//! Search engine for application launcher

use super::AppEntry;
use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;

pub struct SearchEngine {
    apps: Vec<AppEntry>,
    matcher: SkimMatcherV2,
}

impl SearchEngine {
    pub fn new(apps: Vec<AppEntry>) -> Self {
        Self {
            apps,
            matcher: SkimMatcherV2::default(),
        }
    }
    
    pub fn search(&self, query: &str) -> Vec<&AppEntry> {
        if query.is_empty() {
            return self.apps.iter().collect();
        }
        
        let query = query.to_lowercase();
        let mut results: Vec<(&AppEntry, i64)> = Vec::new();
        
        for app in &self.apps {
            if let Some(score) = self.matcher.fuzzy_match(&app.name.to_lowercase(), &query) {
                results.push((app, score));
            }
        }
        
        results.sort_by(|a, b| b.1.cmp(&a.1));
        results.into_iter().map(|(app, _)| app).collect()
    }
}
