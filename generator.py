from PIL import Image
import cv2
import numpy as np

img = cv2.imread('logo.png')
img = cv2.cvtColor(img, cv2.COLOR_BGR2BGRA)

# ========== 第一步：简单方案去掉背景 ==========
# 直接用 RGB 距离，去掉接近白色的背景
rgb = img[:, :, :3].astype(float)
distance = np.sqrt(np.sum((rgb - 255) ** 2, axis=2))

# 简单阈值，去掉白色背景
bg_mask = distance < 50  # 距离白色近的像素
img[bg_mask, 3] = 0

# ========== 第二步：腐蚀处理边缘 ==========
# 只对 alpha 通道的边缘进行腐蚀
alpha = img[:, :, 3]

# 获取 alpha 的边缘区域（不是完全透明，也不是完全不透明的像素）
edge_mask = (alpha > 0) & (alpha < 255)

# 对整个 alpha 通道进行腐蚀，清理毛刺
kernel = cv2.getStructuringElement(cv2.MORPH_ELLIPSE, (5, 5))
eroded_alpha = cv2.erode(alpha, kernel, iterations=1)

# 只在边缘区域应用腐蚀结果
img[edge_mask, 3] = eroded_alpha[edge_mask]

cv2.imwrite('bird_icon.png', img)

# 生成 ICO
pil_img = Image.fromarray(cv2.cvtColor(img, cv2.COLOR_BGRA2RGBA))
pil_img_icon = pil_img.resize((256, 256), Image.Resampling.LANCZOS)
pil_img_icon.save('bird_icon.ico')
print("已保存")