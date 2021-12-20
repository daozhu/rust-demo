import tensorflow as tf
from tensorflow.keras.applications.resnet50 import (decode_predictions, preprocess_input)
from tensorflow.keras.preprocessing import image
import numpy as np

# 加载模型
loaded = tf.saved_model.load('py/resnet50')
infer = loaded.signatures['serving_default']

# 准备推理数据
img = image.load_img('py/daxiang1.jpeg', target_size=(224,224))
x = image.img_to_array(img)

x = np.expand_dims(x, axis=0)

x = preprocess_input(x)

print('>>>>> ', x.shape)

# 推理
preds = infer(tf.constant(x))['predictions'].numpy()
print(preds)

print(decode_predictions(preds, top=3)[0])


print('X: ', x.shape, x.dtype)
# 图片信息保存文件
with open('py/request', 'wb') as f:
    f.write(x.tobytes())
