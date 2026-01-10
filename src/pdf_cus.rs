// https://blog.csdn.net/gitblog_00820/article/details/146532943

use pdf::{
    PdfError, 
    file::FileOptions as PdfFileOptions, 
    object::Resolve as PdfResolve
};

pub fn load_pdf_of_hands_language(path: &str) -> Result<Vec<String>, PdfError> {
    let file = PdfFileOptions::cached().open(path);
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

                        page_rc.contents.iter().for_each(|_| {
                            // 这里可以处理内容流
                            println!("Page {} has content stream", page_num + 1);
                        });

                        let resources = page_rc.resources()?;

                        for (name, xobject_ref) in resources.xobjects.iter() {
                            let xobject = resolver.get(*xobject_ref)?;

                            if let pdf::object::XObject::Image(image) = &*xobject {
                                println!("pageNum {}, 找到图像 {} found, 尺寸: {}x{}", page_num + 1, name, image.width, image.height);
                                // 可以在这里保存图像数据
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
            return Err(e);
        }
    }
}