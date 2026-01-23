#[path ="./fonts_cus.rs"]
mod fonts_cus;

#[path ="./pdf_cus.rs"]
mod pdf_cus;

pub struct MainWindow {
    pdf_file_path: String,
}

impl Default for MainWindow {
    fn default() -> Self {
        Self {
            pdf_file_path: "/home/niutb/文档/jeffreyniu/Hillbilly Elegy by J. D. Vance.pdf".to_string()
        }
    }
}

impl MainWindow {
    fn read_pdf(&self, _path: &str) {
        let result = pdf_cus::load_pdf_of_hands_language(_path);

        match result {
            Ok(info) => {
                for (index, item) in info.iter().enumerate() {
                    println!("PDF Info: {} - {}",index, item);
                }
            }
            Err(e) => {
                println!("Error reading PDF or saving images: {:?}", e);
            }
        }
    }
}