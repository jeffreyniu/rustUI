use std::sync::Arc;
use eframe::egui;

pub fn load_fonts(ctx: &egui::Context) {
    // 创建默认字体配置容器
    let mut fonts = egui::FontDefinitions::default();

    // 注册自定义字体数据（需提前放置simsun.ttc在项目根目录）
    fonts.font_data.insert(
        "wgy_zenhei_font".to_owned(), // 字体标识名
        Arc::new(
            // 使用Arc实现线程安全共享
            egui::FontData::from_owned(
                // 转换字体数据为egui格式
                include_bytes!("../wqy-zenhei.ttc") // 编译时嵌入字体文件
                    .to_vec(), // 转为Vec<u8>
            ),
        ),
    );

    // 配置比例字体家族（用于常规文本）
    fonts
        .families // 访问字体家族集合
        .entry(egui::FontFamily::Proportional) // 获取比例字体入口
        .or_default() // 不存在则创建默认列表
        .insert(0, "wgy_zenhei_font".to_owned()); // 插入到最高优先级

    // 配置等宽字体家族（用于代码/表格）
    fonts
        .families
        .entry(egui::FontFamily::Monospace) // 获取等宽字体入口
        .or_default()
        .push("wgy_zenhei_font".to_owned()); // 追加到列表末尾

    // 将最终配置应用到egui上下文
    ctx.set_fonts(fonts);
}
