//! Performance benchmarks for Chronicle
//! 
//! Run with: cargo bench --manifest-path src-tauri/Cargo.toml
//! 
//! Spec requirements (Section 10):
//! - Vault indexing: Under 5 seconds for 10,000 notes
//! - Search: Under 100ms for results
//! - Note save: Under 50ms

use std::fs;
use std::path::PathBuf;
use std::time::Instant;
use tempfile::TempDir;

// We'll test these manually since Criterion requires additional setup
// This is a simple benchmark runner

fn create_test_vault(note_count: usize) -> (TempDir, PathBuf) {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let vault_path = temp_dir.path().to_path_buf();
    
    // Create notes with varying content and links
    for i in 0..note_count {
        let tags: Vec<String> = (0..3).map(|j| format!("tag{}", (i + j) % 10)).collect();
        let links: Vec<String> = (1..4)
            .map(|j| format!("[[note-{}]]", (i + j * 100) % note_count))
            .collect();
        
        let content = format!(
            r#"---
title: Note {}
tags: [{}]
---

# Note {}

This is note number {} with some content for full-text search.
It contains words like performance, benchmark, and testing.

Links: {}

Some more content to make the note realistic.
Lorem ipsum dolor sit amet, consectetur adipiscing elit.
Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.
"#,
            i,
            tags.join(", "),
            i,
            i,
            links.join(" ")
        );
        
        let path = vault_path.join(format!("note-{}.md", i));
        fs::write(&path, content).expect("Failed to write note");
    }
    
    (temp_dir, vault_path)
}

fn main() {
    println!("Chronicle Performance Benchmarks");
    println!("=================================\n");
    
    // Benchmark 1: Vault indexing with 1000 notes
    println!("Benchmark 1: Index 1,000 notes");
    println!("Target: < 500ms (scaled from 5s for 10k)");
    {
        let (_temp, vault_path) = create_test_vault(1000);
        let db_path = vault_path.join(".chronicle").join("bench.db");
        fs::create_dir_all(db_path.parent().unwrap()).ok();
        
        let start = Instant::now();
        
        // Open database and index
        let db = chronicle_lib::db::schema::Database::open(&db_path)
            .expect("Failed to open db");
        let indexer = chronicle_lib::vault::Indexer::new(vault_path)
            .expect("Failed to create indexer");
        let count = indexer.full_index(&db).expect("Failed to index");
        
        let elapsed = start.elapsed();
        println!("  Indexed {} notes in {:?}", count, elapsed);
        println!("  Result: {}\n", if elapsed.as_millis() < 500 { "PASS" } else { "FAIL" });
    }
    
    // Benchmark 2: Search performance
    println!("Benchmark 2: Full-text search");
    println!("Target: < 100ms");
    {
        let (_temp, vault_path) = create_test_vault(1000);
        let db_path = vault_path.join(".chronicle").join("bench.db");
        fs::create_dir_all(db_path.parent().unwrap()).ok();
        
        let db = chronicle_lib::db::schema::Database::open(&db_path)
            .expect("Failed to open db");
        let indexer = chronicle_lib::vault::Indexer::new(vault_path)
            .expect("Failed to create indexer");
        indexer.full_index(&db).expect("Failed to index");
        
        let conn = db.conn();
        
        // Run multiple searches
        let queries = ["performance", "benchmark testing", "note 500", "lorem ipsum"];
        let mut total_time = std::time::Duration::ZERO;
        
        for query in &queries {
            let start = Instant::now();
            let results = chronicle_lib::db::search::search_notes(&conn, query, 20)
                .expect("Search failed");
            let elapsed = start.elapsed();
            total_time += elapsed;
            println!("  '{}': {} results in {:?}", query, results.len(), elapsed);
        }
        
        let avg = total_time / queries.len() as u32;
        println!("  Average: {:?}", avg);
        println!("  Result: {}\n", if avg.as_millis() < 100 { "PASS" } else { "FAIL" });
    }
    
    // Benchmark 3: Note save performance
    println!("Benchmark 3: Note save (re-index single note)");
    println!("Target: < 50ms");
    {
        let (_temp, vault_path) = create_test_vault(100);
        let db_path = vault_path.join(".chronicle").join("bench.db");
        fs::create_dir_all(db_path.parent().unwrap()).ok();
        
        let db = chronicle_lib::db::schema::Database::open(&db_path)
            .expect("Failed to open db");
        let indexer = chronicle_lib::vault::Indexer::new(vault_path.clone())
            .expect("Failed to create indexer");
        indexer.full_index(&db).expect("Failed to index");
        
        let note_path = vault_path.join("note-50.md");
        let new_content = "# Updated Note\n\nThis note was updated with new content.";
        
        let start = Instant::now();
        fs::write(&note_path, new_content).expect("Failed to write");
        indexer.index_file(&db, &note_path).expect("Failed to re-index");
        let elapsed = start.elapsed();
        
        println!("  Re-indexed single note in {:?}", elapsed);
        println!("  Result: {}\n", if elapsed.as_millis() < 50 { "PASS" } else { "FAIL" });
    }
    
    // Benchmark 4: Large vault indexing (10k notes)
    println!("Benchmark 4: Index 10,000 notes");
    println!("Target: < 5 seconds");
    {
        println!("  Creating 10,000 test notes...");
        let (_temp, vault_path) = create_test_vault(10000);
        let db_path = vault_path.join(".chronicle").join("bench.db");
        fs::create_dir_all(db_path.parent().unwrap()).ok();
        
        let start = Instant::now();
        
        let db = chronicle_lib::db::schema::Database::open(&db_path)
            .expect("Failed to open db");
        let indexer = chronicle_lib::vault::Indexer::new(vault_path)
            .expect("Failed to create indexer");
        let count = indexer.full_index(&db).expect("Failed to index");
        
        let elapsed = start.elapsed();
        println!("  Indexed {} notes in {:?}", count, elapsed);
        println!("  Result: {}\n", if elapsed.as_secs() < 5 { "PASS" } else { "FAIL" });
    }
    
    println!("Benchmarks complete.");
}
