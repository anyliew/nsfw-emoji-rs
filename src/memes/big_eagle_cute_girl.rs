// 导入所需的库和模块
use skia_safe::Color; // 图形库，用于颜色处理

use meme_generator_core::error::Error; // 表情包生成器核心错误类型
use meme_generator_utils::{
    builder::InputImage,     // 输入图像处理
    encoder::GifEncoder,     // GIF 编码器
    image::ImageExt,         // 图像扩展功能
    tools::{load_image, local_date, new_surface}, // 工具函数：加载图像、本地日期、创建画布
};

use crate::{options::NoOptions, register_meme}; // 当前crate的模块

// 主要的表情包生成函数
fn big_eagle_cute_girl(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    // 定义四个帧中图像的位置和尺寸参数 (宽度, 高度, x坐标, y坐标)
    let locs = [
    (70, 70, 193, 20),
    (70, 70, 193, 20),
    (70, 70, 187, 20),
    (70, 70, 182, 20),
    (70, 70, 182, 20),
    (70, 70, 182, 20),
    (70, 70, 186, 17),
    (70, 70, 195, 35),
    (70, 70, 215, 68),
    (70, 70, 227, 65),
    (70, 70, 227, 40),
    (70, 70, 219, 41),
    (70, 70, 223, 48),
    (70, 70, 229, 44),
    (70, 70, 227, 33),
    (70, 70, 211, 28),
    (70, 70, 200, 28),
    (70, 70, 185, 26),
    (70, 70, 176, 20),
    (70, 70, 167, 22),
    (70, 70, 160, 24),
    (70, 70, 160, 31),
    (70, 70, 162, 28),
    (70, 70, 161, 28),
    (70, 70, 154, 30),
    (70, 70, 154, 30),
    (70, 70, 155, 30),
    (70, 70, 155, 30),
    (70, 70, 154, 23),
    (70, 70, 157, 23),
    (70, 70, 152, 22),
    (70, 70, 148, 28),
    (70, 70, 136, 28),
    (70, 70, 123, 19),
    (70, 70, 116, 38),
    (70, 70, 116, 40),
    (70, 70, 116, 40),
    (70, 70, 103, 44),
    (70, 70, 102, 46),
    (70, 70, 100, 48),
    (70, 70, 99, 43),
    (70, 70, 99, 33),
    (70, 70, 98, 35),
    (70, 70, 111, 39),
    (70, 70, 116, 27),
    (70, 70, 103, 19),
    (70, 70, 98, 33),
    (70, 70, 101, 30),
    (70, 70, 113, 29),
    (70, 70, 122, 29),
    (70, 70, 135, 43),
    (70, 70, 143, 45)
    ];
    
    // 获取输入的第一张图像并转换为正方形
    let image = images[0].image.square();

    // 创建GIF编码器
    let mut encoder = GifEncoder::new();
    
    // 循环生成数量帧动画
    for i in 0..52 {
        // 获取当前帧的位置和尺寸参数
        let (w, h, x, y) = locs[i];
        
        // 加载预定义的咖波贴图帧
        let frame = load_image(format!("big_eagle_cute_girl/{i}.png"))?;
        
        // 创建与帧图像相同尺寸的画布
        let mut surface = new_surface(frame.dimensions());
        let canvas = surface.canvas();
        
        // 清空画布为白色背景
        canvas.clear(Color::WHITE);
        
        // 调整输入图像到指定尺寸
        let image = image.resize_exact((w, h));
        
        // 在画布上绘制调整后的输入图像
        canvas.draw_image(&image, (x, y), None);
        
        // 在画布上绘制咖波贴图（覆盖在输入图像上方）
        canvas.draw_image(&frame, (0, 0), None);
        
        // 将当前帧添加到GIF编码器，设置帧间隔为0.04秒
        encoder.add_frame(surface.image_snapshot(), 0.14)?;
    }
    
    // 完成GIF编码并返回字节数据
    Ok(encoder.finish()?)
}

// 注册表情包插件
register_meme! {
    "big_eagle_cute_girl",           // 表情包标识符
    big_eagle_cute_girl,             // 处理函数
    min_images = 1,        // 最少需要1张输入图片
    max_images = 1,        // 最多支持1张输入图片
    keywords = &["大屌萌妹","大吊萌妹","大雕萌妹"], // 搜索关键词
    date_created = local_date(2025, 10, 6), 
    date_modified = local_date(2025, 10, 6),
}