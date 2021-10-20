

pub fn is_pipeline_token(token: &String) -> bool {
    "|".eq(token)
}

#[cfg(test)]
mod tests {
    use super::*;
}
