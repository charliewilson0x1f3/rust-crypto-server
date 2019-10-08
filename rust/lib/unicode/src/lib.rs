fn to_unicode(name: String) -> String
{
	let mut unicode_name = "".to_string();
	let mut vector: Vec<u8> = [].to_vec();
	let chars: Vec<char> = name.chars().collect();
	let mut i = 0;
	while i <= chars.len()-3
	{
		if chars[i] == '%'
		{
			if !(chars[i+1].is_numeric() || chars[i+1] == 'A' || chars[i+1] == 'B' || chars[i+1] == 'C' || chars[i+1] == 'D' || chars[i+1] == 'E' || chars[i+1] == 'F')
			{
				return "ERROR".to_string();
			}
			if !(chars[i+2].is_numeric() || chars[i+2] == 'A' || chars[i+2] == 'B' || chars[i+2] == 'C' || chars[i+2] == 'D' || chars[i+2] == 'E' || chars[i+2] == 'F')
			{
				return "ERROR".to_string();
			}
			let mut c = "".to_string();
			c += &chars[i+1].to_string();
			c += &chars[i+2].to_string();
			vector.push(u8::from_str_radix(&c, 16).unwrap());
			if i+3 <= chars.len()-3
			{
				if chars[i+3] != '%'
				{
					unicode_name += &String::from_utf8(vector.clone()).unwrap();
					vector = [].to_vec();
				}
			}
			else
			{
				unicode_name += &String::from_utf8(vector.clone()).unwrap();
			}
			i += 3;
		}
		else
		{
			unicode_name += &chars[i].to_string();
			i += 1;
			if i > chars.len() - 3
			{
				let end = chars[i].to_string()+&chars[i+1].to_string();
				unicode_name += &end;
			}
		}
	}
	return unicode_name;
}

pub fn unicode(mut name : String) -> String
{
	if str::replace(&name.clone(), "%", "") != name
	{
		name = to_unicode(name);
	}
	return name;
}
