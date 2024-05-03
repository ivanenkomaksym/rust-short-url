#[cfg(test)]
mod tests {
    use rust_short_url::services::hashfunction;
    
    #[test]
    fn test_successful_short_strings() {
        // Arrange
        let str1 = "abc";
        let str2 = "cab";
        let str3 = "bca";

        // Act
        let key1 = hashfunction::hash(str1);
        let key2 = hashfunction::hash(str2);
        let key3 = hashfunction::hash(str3);

        // Assert
        assert_ne!(key1, key2);
        assert_ne!(key2, key3);
    }

    #[test]
    fn test_successful_long_strings() {
        // Arrange
        let str1 = "https://www.google.com/search?client=firefox-b-d&q=Collisions+in+the+cryptographic+hash+functions+are+extremely+unlikely+to+be+found%2C+so+crypto+hashes+are+considered+to+almost+uniquely+identify+their+corresponding+input.+Moreover%2C+it+is+extremely+hard+to+find+an+input+message+that+hashes+to+given+value.";
        let str2 = "https://www.google.com/search?client=firefox-b-d&q=Collisions+1n+the+cryptographic+hash+functions+are+extremely+unlikely+to+be+found%2C+so+crypto+hashes+are+considered+to+almost+uniquely+identify+their+corresponding+input.+Moreover%2C+it+is+extremely+hard+to+find+an+input+message+that+hashes+to+given+value.";

        // Act
        let key1 = hashfunction::hash(str1);
        let key2 = hashfunction::hash(str2);

        // Assert
        assert_ne!(key1, key2);
    }
}