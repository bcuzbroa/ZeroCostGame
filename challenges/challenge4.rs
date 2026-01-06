/* =========================================================
CHALLENGE4: RESULT HANDLING
=========================================================
*/

/*
1. Implement the CommandError Enum
Task: Create an enum named CommandError that derives Debug.
Variants:
- ContainsBang
- ContainsSlash
*/

// TODO: Define your enum here


/*
2. Implement the "check" function
@args: &str
@returns: Result<String, CommandError>

Task:
- If string contains '!', return Err(CommandError::ContainsBang)
- If string contains '/', return Err(CommandError::ContainsSlash)
- Else, return the string wrapped in Ok()
*/

// TODO: Write the check function here


/*
3. Implement the "valid_command" function
@args: &str
@returns: Result<String, CommandError>

Task: Call 'check()' and use pattern matching (match):
- If Ok: return the result.
- If ContainsBang: remove the '!' from the string and return it in an Ok().
- If ContainsSlash: return the error.
*/

// TODO: Write the valid_command function here


// Main Section
fn main() {
   //Implement your tests here
}