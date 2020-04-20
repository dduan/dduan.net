use askama::Template;
use comrak::{self, ComrakOptions};
use crate::article::Article;
use crate::page::Page;
use crate::site::Site;
use crate::templates::*;
use std::error::Error;
use slug;

fn write(text: &str, path: &str, file: &str) -> Result<(), Box<dyn Error>> {
    std::fs::create_dir_all(&path)?;
    std::fs::write(format!("{}/{}", path, file), text)?;
    Ok(())
}

fn build_page(page: &Page, base_url: &str, root_path: &str, output_path: &str) -> Result<(),Box<dyn Error>> {
    let permalink = format!("{}{}", base_url, page.relative_link);
    if let Some(body) = page.read_body(root_path) {
        let template = PageTemplate {
            meta: RenderedMetadata {
                permalink: permalink.to_string(),
                title: page.title.to_string(),
            },
            content: &body,
        };

        write(&template.render()?, &format!("{}{}", output_path, page.relative_link), "index.html")?;
    } else {
        return Err(Box::new(std::io::Error::new(std::io::ErrorKind::InvalidInput, format!("Could not read page {}", page.relative_link))))
    }
    Ok(())
}

fn instantiate_article_template<'a>(article: &'a Article, base_url: &str, root_path: &str) -> Option<ArticleTemplate<'a>> {
    match article.read_body(root_path) {
        None => None,
        Some(markdown) => {
            let mut options = ComrakOptions::default();
            options.github_pre_lang = true;
            let body = comrak::markdown_to_html(&markdown, &options);
            let permalink = format!("{}{}", base_url, article.relative_link);
            let date_string = format!("{}", article.date.format("%Y-%m-%d"));
            let rfc2822date_string = format!("{}", article.date.to_rfc2822());
            Some(ArticleTemplate {
                meta: RenderedMetadata {
                    permalink: permalink.to_string(),
                    title: article.title.to_string(),
                },
                current_url: &article.relative_link,
                date: date_string.to_string(),
                rfc2822_date: rfc2822date_string.to_string(),
                content: body.to_owned(),
                tags: article.tags.iter().map(|tag| RenderedTag::from_name(tag))
                    .collect()
            })
        }
    }
}

fn build_article_list(article_templates: &Vec<ArticleTemplate>, base_url: &str, output_path: &str) -> Result<(), Box<dyn Error>> {
    let template = ArticleListTemplate {
        meta: RenderedMetadata {
            permalink: "/articles".to_string(),
            title: "Daniel Duan's Articles".to_string()
        },
        base_url: base_url,
        items: article_templates,
    };

    write(&template.render()?, &format!("{}{}", output_path, "/articles"), "index.html")?;
    Ok(())
}

fn build_global_feed(article_templates: &Vec<ArticleTemplate>, base_url: &str, output_path: &str) -> Result<(), Box<dyn Error>> {
    let template = GlobalFeedTemplate {
        base_url: base_url,
        items: article_templates,
    };

    write(&template.render()?, output_path, "feed.xml")?;
    Ok(())
}

fn build_tag_list(tag: &str, article_templates: &Vec<ArticleTemplate>, base_url: &str, output_path: &str) -> Result<(), Box<dyn Error>> {
    let slug = slug::slugify(tag);
    let template = TagArticleListTemplate {
        meta: RenderedMetadata {
            permalink: format!("{}/tag/{}", base_url, slug),
            title: format!("Daniel Duan's Articles About {}", tag),
        },
        base_url: base_url,
        tag: RenderedTag::from_name(tag),
        items: article_templates,
    };

    write(&template.render()?, &format!("{}/tag/{}", output_path, slug), "index.html")?;
    Ok(())
}

fn build_tag_feed(tag: &str, article_templates: &Vec<ArticleTemplate>, base_url: &str, output_path: &str) -> Result<(), Box<dyn Error>> {
    let slug = slug::slugify(tag);
    let template = TagFeedTemplate {
        meta: RenderedMetadata {
            permalink: format!("{}/tag/{}/feed.xml", base_url, slug),
            title: format!("Daniel Duan's Articles About {}", tag),
        },
        base_url: base_url,
        tag: RenderedTag::from_name(tag),
        items: article_templates,
    };

    write(&template.render()?, &format!("{}/tag/{}", output_path, slug), "feed.xml")?;
    Ok(())
}

pub fn build_site(site: Site, root_path: &str, output_path: &str) -> Result<(), Box<dyn Error>> {
    if std::fs::metadata(output_path).is_ok() {
        std::fs::remove_dir_all(output_path)?;
        std::fs::create_dir_all(output_path)?;
    }

    let article_templates = site
        .articles
        .iter()
        .filter_map(|article| instantiate_article_template(article, &site.base_url, root_path))
        .collect::<Vec<ArticleTemplate>>();

    build_article_list(&article_templates, &site.base_url, output_path)?;
    build_global_feed(&article_templates, &site.base_url, output_path)?;

    for article_template in article_templates {
        write(
            &article_template.render()?,
            &format!("{}{}", output_path, article_template.current_url),
            "index.html"
        )?;
    }

    for page in &site.pages {
        build_page(page, &site.base_url, root_path, output_path)?;
    }

    for (tag, tagged) in &site.tags {
        let articles = tagged
            .iter()
            .filter_map(|article| instantiate_article_template(article, &site.base_url, root_path))
            .collect::<Vec<ArticleTemplate>>();
        build_tag_list(&tag, &articles, &site.base_url, output_path)?;
        build_tag_feed(&tag, &articles, &site.base_url, output_path)?;
    }
    Ok(())
}