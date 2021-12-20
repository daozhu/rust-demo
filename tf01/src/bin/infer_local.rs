use std::fs;
use std::slice;

use tensorflow::Graph;
use tensorflow::SavedModelBundle;
use tensorflow::SessionOptions;
use tensorflow::SessionRunArgs;
use tensorflow::Tensor;

fn main() {
    // 加载模型
    let export_dir = "py/resnet50";
    let mut graph = Graph::new();
    let opts = SessionOptions::new();
    let sm = SavedModelBundle::load(&opts, &["serve"], &mut graph, export_dir).unwrap();

    // 加载请求参数
    let req = read_req();

    // 进行推理
    let mut step = SessionRunArgs::new();

    let output_op = graph
        .operation_by_name_required("StatefulPartitionedCall")
        .unwrap();
    step.add_target(&output_op);

    let input = graph
        .operation_by_name_required("serving_default_input_1")
        .unwrap();
    step.add_feed(&input, 0, &req);

    let output_t = step.request_fetch(&output_op, 0);

    sm.session.run(&mut step).unwrap();

    //  取出结果
    let output = step.fetch(output_t).unwrap();

    println!("{:?}", output);
}

fn read_req() -> Tensor<f32> {
    let data = fs::read("py/request").unwrap();
    let data = unsafe {
        slice::from_raw_parts(data.as_ptr() as *const f32, data.len() / 4)
    };

    Tensor::new(&[1, 224, 224, 3]).with_values(&data).unwrap()
}