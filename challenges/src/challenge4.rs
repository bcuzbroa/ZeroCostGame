// RESULT HANDLER

// Create the CommandError enum that contains:
// ContainsBang 
// ContainsSlash
#[derive(Debug)]
pub enum CommandError {
    ContainsBang,
    ContainsSlash,
}

/*
return a ContainsBang Error (resp. Slash) if the 
input string contains a '!' (resp. '/')
else : return the string wrapped in some Result type
 */
//Else the Ok(string)
pub fn check(s : &str) -> Result<String, CommandError> {
    if s.contains('!'){
       return Err(CommandError::ContainsBang)
    }
    if s.contains('/'){
       return Err(CommandError::ContainsSlash)
    }
    Ok(s.to_string())
}


/*
if there is a ! is the commande : remove it than return the command
if there is a / in the commande : return std::error::Error
else return the commmand
 */
pub fn valid_command( command: &str) -> Result<String, CommandError> {
    match check(command){
        Ok(c) => Ok(c),
        Err(CommandError::ContainsBang) => Ok(command.replace("!", "")),
        Err(CommandError::ContainsSlash) => Err(CommandError::ContainsSlash)
    }
}

