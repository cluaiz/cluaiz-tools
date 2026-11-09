import os
import json
import subprocess
import sys

def run_cmd(cmd):
    result = subprocess.run(cmd, shell=True, capture_output=True, text=True)
    return result.stdout.strip()

def main():
    # Detect modified package.json files
    # Everything in this push, which can be several commits. A manual run looks at the last commit.
    before = os.environ.get("BEFORE_SHA", "")
    known = before and set(before) != {"0"} and run_cmd(f"git cat-file -t {before}") == "commit"
    diff_output = run_cmd(f"git diff --name-only {before if known else 'HEAD^'} HEAD")
    changed_files = diff_output.split('\n')
    
    packages_to_build = []
    
    for file in changed_files:
        # Only a tool's own manifest, <family>/<id>/package.json; docs hold example manifests
        parts = file.split("/")
        if len(parts) == 3 and parts[0] in ("skills", "plugins", "mcp") and parts[2] == "package.json":
            packages_to_build.append(file)
            
    if not packages_to_build:
        print("No packages changed.")
        # Output empty matrix to GitHub step output
        with open(os.environ['GITHUB_OUTPUT'], 'a') as f:
            f.write("should_run=false\n")
            f.write("matrix={}\n")
        return

    matrix_jobs = []
    
    for pkg_path in packages_to_build:
        if not os.path.exists(pkg_path):
            continue
            
        with open(pkg_path, 'r', encoding='utf-8') as f:
            data = json.load(f)
            
        pkg_id = data.get("id", "unknown")
        build_type = data.get("build_type", "none")
        version = data.get("latest_version", "1.0.0")
        github_action = data.get("github_action", True)
        
        if not github_action:
            print(f"Skipping {pkg_id} as github_action is set to false.")
            continue
        
        # Determine prefix based on category path
        category = pkg_path.split('/')[0].lower()
        prefix = "tool"
        if category == "skills": prefix = "skill"
        elif category == "plugins": prefix = "plugin"
        elif category == "mcp": prefix = "mcp"
        
        # Version specific tag
        tag_name = f"{prefix}-{pkg_id}-v{version}"
        pkg_dir = os.path.dirname(pkg_path)
        
        # The release is named after the tool, such as "Deep Research", never a long title
        title = data.get("name") or pkg_id
        description = data.get("description", "No description provided.")
        
        changelog = "No changelog provided."
        if "versions" in data and version in data["versions"]:
            changelog = data["versions"][version].get("changelog", changelog)
            
        # Build the exact download URLs for the markdown body
        base_url = f"https://github.com/cluaiz/cluaiz-tools/releases/download/{tag_name}"
        zip_link = f"{base_url}/{pkg_id}-files.zip"
        win_link = f"{base_url}/{pkg_id}_windows_x64.dll"
        mac_link = f"{base_url}/lib{pkg_id}_macos_arm64.dylib"
        lin_link = f"{base_url}/lib{pkg_id}_linux_x64.so"
        
        # Short: what it does, what changed, and the install command
        release_body = f"{description}\n\n"
        if changelog != "No changelog provided.":
            release_body += f"**Changes:** {changelog}\n\n"
        release_body += f"Install: `cluaiz {prefix} install {pkg_id}`\n"
        if build_type == "binary":
            release_body += f"- [🪟 Windows Binary]({win_link})\n"
            release_body += f"- [🍎 macOS Binary]({mac_link})\n"
            release_body += f"- [🐧 Linux Binary]({lin_link})\n"
            
        # Helper function to append common fields
        def add_job(job):
            job.update({
                "title": title,
                "release_body": release_body
            })
            matrix_jobs.append(job)
        
        # ALWAYS generate a Package ZIP job for every package
        add_job({
            "id": pkg_id,
            "target": "package-zip",
            "os": "ubuntu-latest",
            "version": version,
            "tag_name": tag_name,
            "pkg_dir": pkg_dir,
            "filename": f"{pkg_id}-files.zip",
            "is_zip": "true"
        })
        
        # Generate matrix permutations based on build_type
        if build_type == "binary":
            builds_os = ["windows", "macos", "linux"]
            if "versions" in data and version in data["versions"]:
                builds_os = data["versions"][version].get("builds_os", ["windows", "macos", "linux"])
                
            if "windows" in builds_os:
                add_job({
                    "id": pkg_id,
                    "target": "windows-x64",
                    "os": "windows-latest",
                    "version": version,
                    "tag_name": tag_name,
                    "pkg_dir": pkg_dir,
                    "filename": f"{pkg_id}_windows_x64.dll",
                    "is_zip": "false"
                })
            if "linux" in builds_os:
                add_job({
                    "id": pkg_id,
                    "target": "linux-x64",
                    "os": "ubuntu-latest",
                    "version": version,
                    "tag_name": tag_name,
                    "pkg_dir": pkg_dir,
                    "filename": f"lib{pkg_id}_linux_x64.so",
                    "is_zip": "false"
                })
            if "macos" in builds_os:
                add_job({
                    "id": pkg_id,
                    "target": "macos-arm64",
                    "os": "macos-15",
                    "version": version,
                    "tag_name": tag_name,
                    "pkg_dir": pkg_dir,
                    "filename": f"lib{pkg_id}_macos_arm64.dylib",
                    "is_zip": "false"
                })
        elif build_type == "wasm":
            add_job({
                "id": pkg_id,
                "target": "wasm32",
                "os": "ubuntu-latest",
                "version": version,
                "tag_name": tag_name,
                "pkg_dir": pkg_dir,
                "filename": f"{pkg_id}_v{version}.wasm",
                "is_zip": "false"
            })
            
    if not matrix_jobs:
        print("No matrix jobs to run.")
        with open(os.environ['GITHUB_OUTPUT'], 'a') as f:
            f.write("should_run=false\n")
            f.write("matrix={}\n")
        return

    matrix = {
        "include": matrix_jobs
    }
    
    matrix_json = json.dumps(matrix)
    print(f"Generated Matrix: {matrix_json}")
    
    with open(os.environ['GITHUB_OUTPUT'], 'a') as f:
        f.write("should_run=true\n")
        f.write(f"matrix={matrix_json}\n")

if __name__ == "__main__":
    main()
