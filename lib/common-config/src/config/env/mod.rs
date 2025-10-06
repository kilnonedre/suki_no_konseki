use std::env;
use std::path::PathBuf;

/// 统一加载顺序（从低到高优先级，后者覆盖前者 *文件之间*；
/// 但**已存在的 OS 环境变量优先于文件**）
///
/// 1) 根 .env（共享）
/// 2) 根 .env.{RUST_ENV}（可选：development / production）
/// 3) 根 .env.local（本地机器覆盖）
/// 4) 服务 .env（服务专属差异）
/// 5) 服务 .env.{RUST_ENV}（可选）
/// 6) 服务 .env.local（服务本地覆盖，优先级最高）
pub fn load_env() {
    use std::collections::HashSet;
    use std::fs;

    // 读取阶段（可选）
    let stage = env::var("RUST_ENV").ok();

    // ===== 计算服务目录与仓库根目录 =====
    let svc_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| ".".into()));

    // 向上查找含 .git 或 [workspace] 的 Cargo.toml，当作仓库根；找不到就用服务目录兜底
    let mut repo_root = svc_dir.clone();
    let mut cur = Some(svc_dir.clone());
    while let Some(dir) = cur {
        let cargo_toml = dir.join("Cargo.toml");
        let is_workspace = fs::read_to_string(&cargo_toml)
            .ok()
            .map(|s| s.contains("[workspace]"))
            .unwrap_or(false);
        if dir.join(".git").exists() || is_workspace {
            repo_root = dir.clone();
            break;
        }
        cur = dir.parent().map(|p| p.to_path_buf());
    }

    // ===== 构建加载顺序（低 -> 高）=====
    let mut files: Vec<PathBuf> = vec![repo_root.join(".env")];
    if let Some(ref s) = stage {
        files.push(repo_root.join(format!(".env.{s}")));
        files.push(repo_root.join(format!(".env.{s}.local")));
    }
    files.push(repo_root.join(".env.local"));

    // 服务层（优先级更高，排在后面）
    files.push(svc_dir.join(".env"));
    if let Some(ref s) = stage {
        files.push(svc_dir.join(format!(".env.{s}")));
        files.push(svc_dir.join(format!(".env.{s}.local")));
    }
    files.push(svc_dir.join(".env.local"));

    // ===== OS 变量优先：记录启动前已有的键 =====
    let os_keys: HashSet<String> = env::vars().map(|(k, _)| k).collect();

    // ===== 依次加载；后面的文件覆盖前面的文件（但不覆盖 OS）=====
    for path in files {
        if !path.exists() {
            continue;
        }
        if let Ok(iter) = dotenvy::from_filename_iter(&path) {
            for pair in iter {
                if let Ok((k, v)) = pair {
                    if !os_keys.contains(&k) {
                        // 允许覆盖前面文件设置的值，从而实现“后者覆盖前者”
                        std::env::set_var(k, v);
                    }
                }
            }
        }
    }
}
