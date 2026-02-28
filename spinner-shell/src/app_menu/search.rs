//! Search engine for application launcher

use super::AppEntry;
use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;
use std::cmp::Reverse;

pub struct SearchEngine {
    apps: Vec<AppEntry>,
    matcher: SkimMatcherV2,
}

#[derive(Debug, Clone)]
pub struct SearchResult {
    pub app: AppEntry,
    pub score: i64,
    pub matched_field: MatchedField,
}

#[derive(Debug, Clone, Copy)]
pub enum MatchedField {
    Name,
    Description,
    Keyword,
}

impl SearchEngine {
    pub fn new(apps: Vec<AppEntry>) -> Self {
        Self {
            apps,
            matcher: SkimMatcherV2::default(),
        }
    }
    
    pub fn search(&self, query: &str) -> Vec<SearchResult> {
        if query.is_empty() {
            return self
                .apps
                .iter()
                .map(|app| SearchResult {
                    app: app.clone(),
                    score: 0,
                    matched_field: MatchedField::Name,
                })
                .collect();
        }
        
        let query = query.to_lowercase();
        let mut results: Vec<SearchResult> = Vec::new();
        
        for app in &self.apps {
            let mut best_score: Option<(i64, MatchedField)> = None;
            
            if let Some(score) = self.matcher.fuzzy_match(&app.name.to_lowercase(), &query) {
                let boosted_score = score * 3;
                best_score = Some((boosted_score, MatchedField::Name));
            }
            
            if let Some(score) = self.matcher.fuzzy_match(&app.description.to_lowercase(), &query) {
                let boosted_score = score * 2;
                if best_score.map_or(true, |(s, _)| boosted_score > s) {
                    best_score = Some((boosted_score, MatchedField::Description));
                }
            }
            
            for keyword in &app.keywords {
                if let Some(score) = self.matcher.fuzzy_match(&keyword.to_lowercase(), &query) {
                    if best_score.map_or(true, |(s, _)| score > s) {
                        best_score = Some((score, MatchedField::Keyword));
                    }
                }
            }
            
            if let Some((score, field)) = best_score {
                results.push(SearchResult {
                    app: app.clone(),
                    score,
                    matched_field: field,
                });
            }
        }
        
        results.sort_by_key(|r| Reverse(r.score));
        
        results
    }
    
    pub fn search_limited(&self, query: &str, limit: usize) -> Vec<SearchResult> {
        self.search(query).into_iter().take(limit).collect()
    }
    
    pub fn update_apps(&mut self, apps: Vec<AppEntry>) {
        self.apps = apps;
    }
    
    pub fn add_app(&mut self, app: AppEntry) {
        self.apps.push(app);
    }
    
    pub fn remove_app(&mut self, exec: &str) {
        self.apps.retain(|a| a.exec != exec);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::app_menu::AppCategory;
    
    fn create_test_apps() -> Vec<AppEntry> {
        vec![
            AppEntry {
                name: "Firefox".to_string(),
                exec: "firefox".to_string(),
                icon: "firefox".to_string(),
                description: "Web Browser".to_string(),
                category: AppCategory::Internet,
                keywords: vec!["web".to_string(), "browser".to_string()],
            },
            AppEntry {
                name: "Files".to_string(),
                exec: "nautilus".to_string(),
                icon: "nautilus".to_string(),
                description: "File manager".to_string(),
                category: AppCategory::Utilities,
                keywords: vec!["file".to_string(), "manager".to_string()],
            },
            AppEntry {
                name: "Terminal".to_string(),
                exec: "gnome-terminal".to_string(),
                icon: "terminal".to_string(),
                description: "Terminal emulator".to_string(),
                category: AppCategory::Utilities,
                keywords: vec!["terminal".to_string(), "console".to_string()],
            },
        ]
    }
    
    #[test]
    fn test_empty_query_returns_all() {
        let apps = create_test_apps();
        let engine = SearchEngine::new(apps.clone());
        
        let results = engine.search("");
        assert_eq!(results.len(), apps.len());
    }
    
    #[test]
    fn test_exact_name_match() {
        let apps = create_test_apps();
        let engine = SearchEngine::new(apps);
        
        let results = engine.search("Firefox");
        assert!(!results.is_empty());
        assert_eq!(results[0].app.name, "Firefox");
    }
    
    #[test]
    fn test_fuzzy_match() {
        let apps = create_test_apps();
        let engine = SearchEngine::new(apps);
        
        let results = engine.search("fire");
        assert!(!results.is_empty());
        assert_eq!(results[0].app.name, "Firefox");
    }
    
    #[test]
    fn test_keyword_match() {
        let apps = create_test_apps();
        let engine = SearchEngine::new(apps);
        
        let results = engine.search("browser");
        assert!(!results.is_empty());
        assert_eq!(results[0].app.name, "Firefox");
    }
}
