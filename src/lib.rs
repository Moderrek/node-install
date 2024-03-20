#[cfg(test)]
mod tests {
    use winreg::enums::HKEY_CURRENT_USER;
    use winreg::RegKey;

    #[test]
    fn get_env_path() {

        let hkcu = RegKey::predef(HKEY_CURRENT_USER);
        let (env, _) = hkcu.create_subkey("Environment").unwrap(); // create_subkey opens with write permissions

        let result: String = env.get_value("Path").unwrap();
        println!("{}", result);
    }
}