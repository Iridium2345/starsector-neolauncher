use sources_manager::ModInfo;
use scraper::{Html, Selector, ElementRef};

#[unsafe(no_mangle)]
pub fn enum_mods(html: &str) -> Result<Vec<ModInfo>, Box<dyn std::error::Error>> {
    let package = Html::parse_document(html);
    let mut mod_list = Vec::new();
    
    let selector = match Selector::parse(".portal_mod_list_item") {
        Ok(s) => s,
        Err(e) => return Err(format!("Failed to parse selector: {}", e).into()),
    };
    
    let elements: Vec<_> = package.select(&selector).collect();
    log::info!("enumerating mods: {} items", elements.len());
    
    for element in elements {
        log::debug!("element HTML: {}", element.inner_html());
        match parse_mod_info(element) {
            Ok(mod_info) => {
                log::info!("Parsed mod: {}", mod_info.name());
                mod_list.push(mod_info);
            }
            Err(e) => {
                log::warn!("Failed to parse mod element: {}", e);
                // 继续处理其他元素，不要因为一个元素解析失败就停止
            }
        }
    }
    
    log::info!("Successfully parsed {} mods", mod_list.len());
    Ok(mod_list)
}

fn parse_mod_info(element: ElementRef) -> Result<ModInfo, Box<dyn std::error::Error>> {
    // 从元素属性中提取模组信息
    let name = element
        .value()
        .attr("mod_name_cn")
        .or_else(|| element.value().attr("mod_name_en"))
        .unwrap_or("未知模组")
        .trim()
        .to_string();
    
    let version = element
        .value()
        .attr("mod_release_version")
        .unwrap_or("未知版本")
        .trim()
        .to_string();
    
    let author = element
        .value()
        .attr("mod_author")
        .unwrap_or("未知作者")
        .trim()
        .to_string();
    
    let date = element
        .value()
        .attr("mod_update_date")
        .unwrap_or("未知日期")
        .trim()
        .to_string();
    
    // 提取链接URL
    let url = element
        .select(&Selector::parse("a").unwrap())
        .next()
        .and_then(|a| a.value().attr("href"))
        .map(|href| {
            if href.starts_with("http") {
                href.to_string()
            } else {
                format!("https://www.fossic.org/{}", href)
            }
        })
        .unwrap_or_else(|| "".to_string());
    
    if name.is_empty() || name == "未知模组" {
        return Err("模组名称为空".into());
    }
    
    log::debug!("解析模组信息: 名称={}, 版本={}, 作者={}, 日期={}", name, version, author, date);
    
    Ok(ModInfo::new(name, version, url, date, author))
}