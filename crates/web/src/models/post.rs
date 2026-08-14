#[derive(Debug, Clone)]
pub struct BlogPost {
    pub title: &'static str,
    pub slug: &'static str,
    pub date: &'static str,
    pub summary: &'static str,
    pub tags: &'static [&'static str],
}

#[derive(Debug, Clone)]
pub struct Project {
    pub name: &'static str,
    pub description: &'static str,
    pub url: &'static str,
    pub tech: &'static [&'static str],
}

// TODO: I aint writing tailwind shit,
// imma do markdown parser soon
pub static BLOG_POSTS: &[BlogPost] = &[];

pub static PROJECTS: &[Project] = &[];
