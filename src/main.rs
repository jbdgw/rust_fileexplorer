use clap::Parser;
use fexplorer::{
    cli::{
        self, parse_entry_kinds, parse_sort_key, parse_sort_order, Cli, Commands, ProfileCommand,
    },
    config::Config,
    errors::{FsError, Result},
    fs::{
        filters::{
            AndPredicate, CategoryFilter, DateFilter, ExtensionFilter, GlobFilter, KindFilter,
            Predicate, RegexFilter, SizeFilter,
        },
        size::{compute_dir_sizes, get_top_by_size, update_entries_with_dir_sizes},
        traverse::{walk, walk_no_filter, TraverseConfig},
    },
    models::{Entry, EntryKind, OutputFormat, SortKey, SortOrder},
    output::{
        csvw::CsvFormatter,
        format::OutputSink,
        json::{JsonFormatter, NdjsonFormatter},
        pretty::{PrettyFormatter, TreeFormatter},
    },
};
use std::io;

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::List {
            path,
            sort,
            order,
            dirs_first,
            common,
        } => {
            let config = build_traverse_config(&common, cli.quiet);
            let predicate = build_predicate_from_common(&common)?;

            let mut entries = if let Some(pred) = &predicate {
                walk(&path, &config, Some(pred.as_ref()))?
            } else {
                walk_no_filter(&path, &config)?
            };

            // Sort if requested
            if let Some(sort_key_str) = sort {
                let sort_key = parse_sort_key(&sort_key_str)?;
                let sort_order = parse_sort_order(&order)?;
                sort_entries(&mut entries, sort_key, sort_order, dirs_first);
            }

            output_entries(&entries, &common, cli.no_color)?;
        }

        Commands::Tree {
            path,
            dirs_first,
            common,
        } => {
            let config = build_traverse_config(&common, cli.quiet);
            let entries = walk_no_filter(&path, &config)?;

            // For tree view, use TreeFormatter
            let stdout = io::stdout();
            let stdout_lock = stdout.lock();
            let mut formatter = TreeFormatter::new(Box::new(stdout_lock), cli.no_color, dirs_first);
            formatter.write_tree(&entries)?;
        }

        Commands::Find {
            path,
            names,
            regex,
            ext,
            min_size,
            max_size,
            after,
            before,
            kind,
            category,
            common,
        } => {
            let config = build_traverse_config(&common, cli.quiet);

            // Build combined predicate
            let mut predicates: Vec<Box<dyn Predicate>> = Vec::new();

            if !names.is_empty() {
                predicates.push(Box::new(GlobFilter::new(&names)?));
            }

            if let Some(ref pattern) = regex {
                predicates.push(Box::new(RegexFilter::new(pattern)?));
            }

            if !ext.is_empty() {
                predicates.push(Box::new(ExtensionFilter::new(&ext)));
            }

            if min_size.is_some() || max_size.is_some() {
                predicates.push(Box::new(SizeFilter::new(
                    min_size.as_deref(),
                    max_size.as_deref(),
                )?));
            }

            if after.is_some() || before.is_some() {
                predicates.push(Box::new(DateFilter::new(
                    after.as_deref(),
                    before.as_deref(),
                )?));
            }

            if !kind.is_empty() {
                let kinds = parse_entry_kinds(&kind)?;
                predicates.push(Box::new(KindFilter::new(&kinds)));
            }

            if let Some(cat) = category {
                predicates.push(Box::new(CategoryFilter::new(&cat)));
            }

            let entries = if !predicates.is_empty() {
                let combined = AndPredicate::new(predicates);
                walk(&path, &config, Some(&combined))?
            } else {
                walk_no_filter(&path, &config)?
            };
            output_entries(&entries, &common, cli.no_color)?;
        }

        Commands::Size {
            path,
            top,
            aggregate,
            du,
            common,
        } => {
            let config = build_traverse_config(&common, cli.quiet);
            let mut entries = walk_no_filter(&path, &config)?;

            if aggregate || du {
                // Compute directory sizes
                let dir_sizes = compute_dir_sizes(&entries);
                update_entries_with_dir_sizes(&mut entries, &dir_sizes);
            }

            // Filter to top N if requested
            if let Some(n) = top {
                entries = get_top_by_size(&entries, n);
            }

            // Sort by size descending for size command
            entries.sort_by(|a, b| b.size.cmp(&a.size));

            output_entries(&entries, &common, cli.no_color)?;
        }

        #[cfg(feature = "grep")]
        Commands::Grep {
            path,
            pattern,
            regex,
            case_insensitive,
            ext,
            context,
            line_numbers,
            common,
        } => {
            use fexplorer::fs::content::{search_files, ContentSearcher};

            let config = build_traverse_config(&common, cli.quiet);

            // Build extension filter if provided
            let mut predicates: Vec<Box<dyn Predicate>> = Vec::new();
            if !ext.is_empty() {
                predicates.push(Box::new(ExtensionFilter::new(&ext)));
            }

            // Get files to search
            let entries = if !predicates.is_empty() {
                let combined = AndPredicate::new(predicates);
                walk(&path, &config, Some(&combined))?
            } else {
                walk_no_filter(&path, &config)?
            };

            // Create searcher
            let searcher =
                ContentSearcher::new(&pattern, regex, case_insensitive, context, line_numbers)?;

            // Search files
            let matches = search_files(&entries, &searcher)?;

            // Output matches
            if matches.is_empty() {
                if !cli.quiet {
                    println!("No matches found");
                }
            } else {
                for m in &matches {
                    if line_numbers {
                        println!(
                            "{}:{}:{}: {}",
                            m.entry.path.display(),
                            m.line_number,
                            m.column,
                            m.matched_text
                        );
                    } else {
                        println!("{}: {}", m.entry.path.display(), m.matched_text);
                    }

                    // Print context if requested
                    if !m.context_before.is_empty() {
                        for (i, line) in m.context_before.iter().enumerate() {
                            let line_num = m.line_number - m.context_before.len() + i;
                            println!("  {}-  {}", line_num, line);
                        }
                    }
                    if !m.context_after.is_empty() {
                        for (i, line) in m.context_after.iter().enumerate() {
                            let line_num = m.line_number + i + 1;
                            println!("  {}+  {}", line_num, line);
                        }
                    }
                }

                println!(
                    "\nFound {} matches in {} files",
                    matches.len(),
                    matches
                        .iter()
                        .map(|m| &m.entry.path)
                        .collect::<std::collections::HashSet<_>>()
                        .len()
                );
            }
        }

        #[cfg(feature = "dedup")]
        Commands::Duplicates {
            path,
            min_size,
            summary,
            common,
        } => {
            use fexplorer::fs::dedup::{find_duplicates, DuplicateStats};
            use fexplorer::util::parse_size;

            let config = build_traverse_config(&common, cli.quiet);
            let entries = walk_no_filter(&path, &config)?;

            // Parse min size
            let min_size_bytes = parse_size(&min_size)?;

            // Find duplicates
            let groups = find_duplicates(&entries, min_size_bytes)?;

            if groups.is_empty() {
                if !cli.quiet {
                    println!("No duplicate files found");
                }
            } else if summary {
                // Show summary statistics
                let stats = DuplicateStats::from_groups(&groups);
                println!("Duplicate Files Summary:");
                println!("  Total duplicate groups: {}", stats.total_groups);
                println!("  Total duplicate files: {}", stats.total_files);
                println!(
                    "  Total wasted space: {}",
                    humansize::format_size(stats.total_wasted_space, humansize::BINARY)
                );
                println!(
                    "  Largest group wasted space: {}",
                    humansize::format_size(stats.largest_group_size, humansize::BINARY)
                );
                println!("  Largest group file count: {}", stats.largest_group_count);
            } else {
                // Show detailed groups
                for (i, group) in groups.iter().enumerate() {
                    println!(
                        "\nDuplicate Group #{} (hash: {}...)",
                        i + 1,
                        &group.hash[..8]
                    );
                    println!(
                        "  File size: {}",
                        humansize::format_size(group.size, humansize::BINARY)
                    );
                    println!("  Count: {} files", group.count);
                    println!(
                        "  Wasted space: {}",
                        humansize::format_size(group.wasted_space, humansize::BINARY)
                    );
                    println!("  Files:");
                    for entry in &group.entries {
                        println!("    - {}", entry.path.display());
                    }
                }

                let stats = DuplicateStats::from_groups(&groups);
                println!(
                    "\nTotal: {} groups, {} files, {} wasted",
                    stats.total_groups,
                    stats.total_files,
                    humansize::format_size(stats.total_wasted_space, humansize::BINARY)
                );
            }
        }

        #[cfg(feature = "git")]
        Commands::Git {
            path,
            status,
            since,
            common,
        } => {
            use fexplorer::fs::git::{
                enrich_with_git_status, get_changed_since, is_git_repo, GitStatus,
            };

            // Check if path is in a git repository
            if !is_git_repo(&path) {
                return Err(FsError::InvalidFormat {
                    format: format!("{} is not in a git repository", path.display()),
                });
            }

            let config = build_traverse_config(&common, cli.quiet);
            let mut entries = walk_no_filter(&path, &config)?;

            // If "since" is specified, filter to only changed files
            if let Some(since_ref) = since {
                let changed_files = get_changed_since(&path, &since_ref)?;
                let changed_set: std::collections::HashSet<_> = changed_files.into_iter().collect();
                entries.retain(|e| changed_set.contains(&e.path));
            }

            // Enrich entries with git status
            let git_entries = enrich_with_git_status(&entries, &path)?;

            // Collect status counts before filtering
            let status_counts = if !cli.quiet {
                Some(
                    git_entries
                        .iter()
                        .fold(std::collections::HashMap::new(), |mut acc, ge| {
                            *acc.entry(ge.status).or_insert(0) += 1;
                            acc
                        }),
                )
            } else {
                None
            };

            // Filter by status if requested
            let filtered_entries: Vec<_> = if let Some(status_filter) = status {
                let filter_status = match status_filter {
                    cli::GitStatusFilter::Untracked => GitStatus::Untracked,
                    cli::GitStatusFilter::Modified => GitStatus::Modified,
                    cli::GitStatusFilter::Staged => GitStatus::Staged,
                    cli::GitStatusFilter::Conflict => GitStatus::Unmerged,
                    cli::GitStatusFilter::Ignored => GitStatus::Ignored,
                    cli::GitStatusFilter::Clean => GitStatus::Clean,
                };

                git_entries
                    .into_iter()
                    .filter(|ge| ge.status == filter_status)
                    .map(|ge| ge.entry)
                    .collect()
            } else {
                git_entries
                    .into_iter()
                    .filter(|ge| ge.status != GitStatus::Clean)
                    .map(|ge| ge.entry)
                    .collect()
            };

            output_entries(&filtered_entries, &common, cli.no_color)?;

            if let Some(status_counts) = status_counts {
                println!("\nGit Status Summary:");
                for (status, count) in status_counts {
                    println!("  {}: {}", status.to_str(), count);
                }
            }
        }

        #[cfg(feature = "tui")]
        Commands::Interactive { path } => {
            use fexplorer::tui::{ui, App};

            let mut app = App::new(path)?;
            ui::run(&mut app).map_err(|e| FsError::IoError {
                context: "TUI error".to_string(),
                source: e,
            })?;
        }

        #[cfg(feature = "trends")]
        Commands::Snapshot {
            path: _,
            description: _,
        } => {
            println!("🚧 Snapshot command - Implementation coming in Phase 4!");
            println!("This will save filesystem state for trend analysis.");
        }

        #[cfg(feature = "trends")]
        Commands::Trends {
            path: _,
            since: _,
            chart: _,
        } => {
            println!("🚧 Trends command - Implementation coming in Phase 4!");
            println!("This will analyze filesystem growth over time.");
        }

        Commands::Completions { shell } => {
            use clap::CommandFactory;
            use clap_complete::{generate, Shell as CompShell};

            let mut cmd = Cli::command();
            let shell_type = match shell {
                cli::Shell::Bash => CompShell::Bash,
                cli::Shell::Zsh => CompShell::Zsh,
                cli::Shell::Fish => CompShell::Fish,
                cli::Shell::Powershell => CompShell::PowerShell,
                cli::Shell::Elvish => CompShell::Elvish,
            };

            generate(shell_type, &mut cmd, "fexplorer", &mut io::stdout());
        }

        Commands::Profiles { command } => match command {
            ProfileCommand::List => {
                let config = Config::load()?;
                let names = config.profile_names();

                if names.is_empty() {
                    println!("No profiles configured.");
                    println!("Run 'fexplorer profiles init' to create example profiles.");
                } else {
                    println!("Available profiles:");
                    for name in names {
                        if let Some(profile) = config.get_profile(&name) {
                            if let Some(desc) = &profile.description {
                                println!("  {} - {}", name, desc);
                            } else {
                                println!("  {} ({})", name, profile.command);
                            }
                        }
                    }
                }
            }

            ProfileCommand::Show { name } => {
                let config = Config::load()?;
                if let Some(profile) = config.get_profile(&name) {
                    println!("Profile: {}", name);
                    if let Some(desc) = &profile.description {
                        println!("Description: {}", desc);
                    }
                    println!("Command: {}", profile.command);
                    println!("Arguments:");
                    for (key, value) in &profile.args {
                        println!("  {} = {}", key, value);
                    }
                } else {
                    eprintln!("Profile '{}' not found", name);
                }
            }

            ProfileCommand::Init => {
                Config::init()?;
                let config_path = Config::config_file_path()?;
                println!("Initialized config with example profiles at:");
                println!("{}", config_path.display());
            }
        },

        Commands::Run {
            profile,
            path,
            args,
        } => {
            let config = Config::load()?;
            let profile_def =
                config
                    .get_profile(&profile)
                    .ok_or_else(|| FsError::InvalidFormat {
                        format: format!("Profile '{}' not found", profile),
                    })?;

            // Use path from CLI args if provided, otherwise use current directory
            let target_path = path.unwrap_or_else(|| std::path::PathBuf::from("."));

            // Parse additional CLI args as key-value overrides
            let mut override_args = std::collections::HashMap::new();
            let mut i = 0;
            while i < args.len() {
                if let Some(key) = args.get(i).and_then(|s| s.strip_prefix("--")) {
                    if let Some(value) = args.get(i + 1) {
                        override_args.insert(key.to_string(), serde_json::json!(value));
                        i += 2;
                    } else {
                        i += 1;
                    }
                } else {
                    i += 1;
                }
            }

            // Merge profile args with overrides
            let mut merged_args = profile_def.args.clone();
            for (key, value) in override_args {
                merged_args.insert(key, value);
            }

            if !cli.quiet {
                println!("Running profile: {}", profile);
                if let Some(desc) = &profile_def.description {
                    println!("Description: {}", desc);
                }
                println!();
            }

            // Execute the command based on profile
            match profile_def.command.as_str() {
                "find" => {
                    let mut predicates: Vec<Box<dyn Predicate>> = Vec::new();
                    let config = build_traverse_config(&cli::CommonArgs::default(), cli.quiet);

                    // Build predicates from merged args
                    if let Some(names) = merged_args.get("names").and_then(|v| v.as_array()) {
                        let names: Vec<String> = names
                            .iter()
                            .filter_map(|v| v.as_str().map(String::from))
                            .collect();
                        if !names.is_empty() {
                            predicates.push(Box::new(GlobFilter::new(&names)?));
                        }
                    }

                    if let Some(ext) = merged_args.get("ext").and_then(|v| v.as_array()) {
                        let extensions: Vec<String> = ext
                            .iter()
                            .filter_map(|v| v.as_str().map(String::from))
                            .collect();
                        if !extensions.is_empty() {
                            predicates.push(Box::new(ExtensionFilter::new(&extensions)));
                        }
                    }

                    if let Some(min) = merged_args.get("min_size").and_then(|v| v.as_str()) {
                        let max = merged_args.get("max_size").and_then(|v| v.as_str());
                        predicates.push(Box::new(SizeFilter::new(Some(min), max)?));
                    }

                    if let Some(after) = merged_args.get("after").and_then(|v| v.as_str()) {
                        let before = merged_args.get("before").and_then(|v| v.as_str());
                        predicates.push(Box::new(DateFilter::new(Some(after), before)?));
                    }

                    if let Some(category) = merged_args.get("category").and_then(|v| v.as_str()) {
                        predicates.push(Box::new(CategoryFilter::new(category)));
                    }

                    let entries = if !predicates.is_empty() {
                        let combined = AndPredicate::new(predicates);
                        walk(&target_path, &config, Some(&combined))?
                    } else {
                        walk_no_filter(&target_path, &config)?
                    };

                    let common = cli::CommonArgs::default();
                    output_entries(&entries, &common, cli.no_color)?;
                }
                "list" => {
                    let config = build_traverse_config(&cli::CommonArgs::default(), cli.quiet);
                    let entries = walk_no_filter(&target_path, &config)?;
                    let common = cli::CommonArgs::default();
                    output_entries(&entries, &common, cli.no_color)?;
                }
                "size" => {
                    let config = build_traverse_config(&cli::CommonArgs::default(), cli.quiet);
                    let mut entries = walk_no_filter(&target_path, &config)?;

                    let dir_sizes = compute_dir_sizes(&entries);
                    update_entries_with_dir_sizes(&mut entries, &dir_sizes);
                    entries.sort_by(|a, b| b.size.cmp(&a.size));

                    if let Some(top) = merged_args
                        .get("top")
                        .and_then(|v| v.as_u64())
                        .map(|v| v as usize)
                    {
                        entries = get_top_by_size(&entries, top);
                    }

                    let common = cli::CommonArgs::default();
                    output_entries(&entries, &common, cli.no_color)?;
                }
                cmd => {
                    return Err(FsError::InvalidFormat {
                        format: format!("Unsupported profile command: {}", cmd),
                    });
                }
            }
        }

        #[cfg(feature = "watch")]
        Commands::Watch {
            path,
            events,
            format,
        } => {
            use fexplorer::fs::watch::FileWatcher;

            let watcher = FileWatcher::new(events);

            // For watch, we output events as they come
            match format.as_str() {
                "ndjson" => {
                    watcher.watch(&path, |event| {
                        if let Ok(json) = serde_json::to_string(&event) {
                            println!("{}", json);
                        }
                    })?;
                }
                _ => {
                    watcher.watch(&path, |event| {
                        println!("{:?}", event);
                    })?;
                }
            }
        }

        #[cfg(feature = "plugins")]
        Commands::Plugins { command: _ } => {
            println!("🚧 Plugins command - Implementation coming in Phase 4!");
            println!("This will manage loadable filter plugins.");
        }
    }

    Ok(())
}

fn build_traverse_config(common: &cli::CommonArgs, quiet: bool) -> TraverseConfig {
    TraverseConfig {
        max_depth: common.max_depth,
        follow_symlinks: common.follow_symlinks,
        include_hidden: common.hidden,
        respect_gitignore: !common.no_gitignore,
        #[cfg(feature = "parallel")]
        threads: common.threads,
        #[cfg(not(feature = "parallel"))]
        threads: 1,
        quiet,
    }
}

fn build_predicate_from_common(_common: &cli::CommonArgs) -> Result<Option<Box<dyn Predicate>>> {
    // For basic list, we don't apply additional predicates
    // They're applied in specific subcommands
    Ok(None)
}

fn sort_entries(entries: &mut [Entry], key: SortKey, order: SortOrder, dirs_first: bool) {
    entries.sort_by(|a, b| {
        // Apply dirs_first if requested
        if dirs_first {
            match (a.kind, b.kind) {
                (EntryKind::Dir, EntryKind::File) => return std::cmp::Ordering::Less,
                (EntryKind::File, EntryKind::Dir) => return std::cmp::Ordering::Greater,
                _ => {}
            }
        }

        let cmp = match key {
            SortKey::Name => a.name.cmp(&b.name),
            SortKey::Size => a.size.cmp(&b.size),
            SortKey::Mtime => a.mtime.cmp(&b.mtime),
            SortKey::Kind => format!("{:?}", a.kind).cmp(&format!("{:?}", b.kind)),
        };

        match order {
            SortOrder::Asc => cmp,
            SortOrder::Desc => cmp.reverse(),
        }
    });
}

fn output_entries(entries: &[Entry], common: &cli::CommonArgs, no_color: bool) -> Result<()> {
    // Check if template export is requested
    #[cfg(feature = "templates")]
    if let Some(template_name) = &common.template {
        use fexplorer::output::templates::{export_with_template, TemplateFormat};

        let format = template_name.parse::<TemplateFormat>().map_err(|e| {
            FsError::InvalidFormat {
                format: e.to_string(),
            }
        })?;

        let stdout = io::stdout();
        let mut stdout_lock = stdout.lock();

        return export_with_template(&mut stdout_lock, entries, &format, None);
    }

    let format = common.output_format()?;
    let columns = common.columns()?;

    let stdout = io::stdout();
    let stdout_lock = stdout.lock();

    let mut sink: Box<dyn OutputSink> = match format {
        OutputFormat::Pretty => Box::new(PrettyFormatter::new(
            Box::new(stdout_lock),
            columns,
            no_color,
        )),
        OutputFormat::Json => Box::new(JsonFormatter::new(Box::new(stdout_lock))),
        OutputFormat::Ndjson => Box::new(NdjsonFormatter::new(Box::new(stdout_lock))),
        OutputFormat::Csv => Box::new(CsvFormatter::new(Box::new(stdout_lock), columns)?),
    };

    for entry in entries {
        sink.write(entry)?;
    }

    sink.finish()?;
    Ok(())
}
