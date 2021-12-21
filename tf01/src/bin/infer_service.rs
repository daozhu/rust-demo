use tensorflow::Graph;
use tensorflow::SavedModelBundle;
use tensorflow::SessionOptions;
use tensorflow::SessionRunArgs;
use tensorflow::Tensor;

use tokio::runtime::Builder;
use tonic::Request;
use tonic::Response;
use tonic::Status;
use tonic::transport::Server;
use async_trait::async_trait;

use tf01::infer_server;
use tf01::InferRequest;
use tf01::InferReponse;

struct InferImpl {
    model: SavedModelBundle,
    graph: Graph,
}

impl InferImpl {
    fn new(export_dir: &'static str) -> InferImpl {
        let mut graph = Graph::new();
        let opts = SessionOptions::new();
        let model = SavedModelBundle::load(&opts, &["serve"], &mut graph, export_dir).unwrap();
        
        InferImpl {model, graph}
    }

    fn infer_impl(&self, req: Tensor<f32>) -> Tensor<f32> {
        let mut step = SessionRunArgs::new();

        let output_op = self.graph
            .operation_by_name_required("StatefulPartitionedCall")
            .unwrap();
        step.add_target(&output_op);
    
        let input = self.graph
            .operation_by_name_required("serving_default_input_1")
            .unwrap();
        step.add_feed(&input, 0, &req);    
        
        let output_t = step.request_fetch(&output_op, 0);
    
        self.model.session.run(&mut step).unwrap();
    
        step.fetch(output_t).unwrap()
    }
} 

#[async_trait]
impl infer_server::Infer for InferImpl {
    async fn infer(
        &self,
        req: Request<InferRequest>, 
    ) -> Result<Response<InferReponse>, Status> {
        let req = req.into_inner();
        let request = Tensor::new(&req.shape).with_values(&req.data).unwrap();
        let output = self.infer_impl(request);
        let response = InferReponse{
            shape: output.dims().into(),
            data: output.to_vec(),
        };

        Ok(Response::new(response))
    }
}

fn main() {
    let rt = Builder::new_current_thread().enable_all().build().unwrap();

    rt.block_on(async {
        let addr = "0.0.0.0:8500";
        println!("监听端口: {}", addr);
        let addr = addr.parse().unwrap();
        let infer = InferImpl::new("py/resnet50");
        let server = infer_server::InferServer::new(infer);

        Server::builder().add_service(server).serve(addr).await.unwrap();
    });
}