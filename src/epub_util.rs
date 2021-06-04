extern crate epub_builder;

use super::scrape::Chapter;
use epub_builder::EpubBuilder;
use epub_builder::EpubContent;
use epub_builder::ReferenceType;
use epub_builder::ZipLibrary;

//TODO : sort chapters into 100 per volume
//TODO : create a coverpage with image
pub fn create_epub(chapter_list: Vec<Chapter>) -> epub_builder::Result<()> {
    let novel_name = &chapter_list.clone().get(0).unwrap().novel_name.to_string();
    let mut builder = EpubBuilder::new(ZipLibrary::new()?)?;

    chapter_list.into_iter().for_each(|x| {
        let con = std::fs::read_to_string(format!("./{}/{}", x.novel_name, x.name))
            .expect("Unable to read file");

        let content = format!(
            r#"<?xml version="1.0" encoding="UTF-8"?>
		<html xmlns="http://www.w3.org/1999/xhtml" xmlns:epub="http://www.idpf.org/2007/ops">
		
		<body>
		{}
		</body>
		</html>"#,
            con
        );

        builder
            .add_content(
                EpubContent::new(format!("{}.xhtml", x.name), content.as_bytes())
                    .title(x.name)
                    .reftype(ReferenceType::Text),
            )
            .unwrap();
    });
    let mut epub: Vec<u8> = vec![];

    builder.generate(&mut epub)?;

    std::fs::write(format!("./{}.epub", novel_name), epub).expect("Unable to write epub");
    Ok(())
}