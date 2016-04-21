pub fn reply(words : &str) -> &str {
	match words {
		"" => "Fine. Be that way!",
		_ if words.ends_with("?") => "Sure.",
		_ if *words == words.to_uppercase() => "Whoa, chill out!",
		_ => "Whatever."
	}
}