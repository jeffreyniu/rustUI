// https://blog.csdn.net/gitblog_00820/article/details/146532943
// integrate poppler to extract text content from PDF

mod image_cus;

use std::error::Error;

use pdf::{
    file::FileOptions as PdfFileOptions, 
    object::Resolve as PdfResolve
};

pub fn load_pdf_of_hands_language(path: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let file = PdfFileOptions::cached().open(path);
    let document = poppler::PopplerDocument::new_from_file(path, None)?;

    match file {
        Ok(file) => {
            println!("Successfully opened PDF file: {}", path);
            // Here you would add the actual PDF processing logic
            let mut pdf_info: Vec<String> = Vec::new();

            pdf_info.push(file.version().unwrap());

            let num_pages=file.num_pages();
            pdf_info.push(format!("Number of pages: {}", num_pages));

            let resolver  = file.resolver();
    
            // 获取指定页面
            let mut page_num: u32 = 8;
            while page_num < num_pages {
                let page = file.pages().nth(page_num as usize);

                match page {
                    Some(Ok(page_rc)) => {
                        //pdf_info.push(format!("Page {} loaded successfully.", page_num));

                        let poppler_page = document.get_page(page_num as usize).ok_or("Failed to get page")?;
                        let text = poppler_page.get_text().unwrap_or_default();
                        println!("Page {} text content:\n{}", page_num + 1, text.len());

                        let resources = page_rc.resources()?;

                        for (name, xobject_ref) in resources.xobjects.iter() {
                            let xobject = resolver.get(*xobject_ref)?;

                            if let pdf::object::XObject::Image(image) = &*xobject {
                                println!("pageNum {}, 找到图像 {} found, 尺寸: {}x{}", page_num + 1, name, image.width, image.height);
                                let image_data = image.image_data(&resolver)?; // 获取图像数据
                                let vec_image_data = image_data.to_vec();
                                let path=format!("/home/niutb/文档/jeffreyniu/output_page{}.png", page_num + 1);
                                if let Err(e) = image_cus::save_image_as_png(vec_image_data, image.width, image.height, &path) {
                                    println!("Failed to save image on page {}: {:?}", page_num + 1, e);
                                }
                            }
                        }

                    }
                    Some(Err(e)) => {
                        pdf_info.push(format!("Error loading page {}: {:?}", page_num + 1, e));
                    }
                    None => {
                        pdf_info.push(format!("Page {} does not exist.", page_num + 1));
                    }
                }
               
                page_num += 1;
            }

            return Ok(pdf_info);
        }
        Err(e) => {
            println!("Failed to open PDF file: {}", path);
            return Err(Box::new(e));
        }
    }
}