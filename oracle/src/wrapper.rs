
pub fn wrapper(base_path: &str, wrapper_main: &str, id : &str) -> String {

        let file_path = format!("{}/challenge{}.rs", base_path, id);
        let player_code = std::fs::read_to_string(&file_path)
            .expect(&format!("Could not read {}", file_path));

        // Create a temporary source file including the player's code and our test main
        let temp_src = format!("temp_challenge_{}.rs", id);
        let full_code = format!("{}\n\n{}", player_code, wrapper_main);
        std::fs::write(&temp_src, full_code).unwrap();

        let bin_name = format!("./temp_bin_{}", id); //binary_name

        // I. Compiling the code to the temp directory
        let compile_output = std::process::Command::new("rustc")
            .arg(&temp_src) //path
            .arg("-o")
            .arg(&bin_name) //output name
            .output()
            .expect("Failed to run rustc");

        // II. Processing the compilation Error
        if !compile_output.status.success(){
            let err = String::from_utf8_lossy(&compile_output.stderr);
            println!("❌ Compilation Error ❌ in challenge {}:\n{}", id, err);
            std::fs::remove_file(temp_src).ok();
            std::process::exit(1);  
        }

        // III. Running the binary
        let run_output = std::process::Command::new(&bin_name)
            .output()
            .expect("Failed to execute binary");

        
        // IV. Cleanup
        let _ = std::fs::remove_file(temp_src);
        let _ = std::fs::remove_file(&bin_name);

        String::from_utf8_lossy(&run_output.stdout).trim().to_string()
    }
