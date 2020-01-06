impl ::std::fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match self {
            ErrorKind::Varlink_Error => write!(f, "Varlink Error"),
            ErrorKind::VarlinkReply_Error => write!(f, "Varlink error reply"),
            // Print the message inside of the error
            ErrorKind::Error(v) => write!(
                f,
                "{}",
                v.as_ref()
                    .map_or_else(|| format!("{:#?}", v), |v| v.message.clone())
            ),
            // Print that RPC requires the "more" flag
            ErrorKind::RequiresMore(_) => write!(
                f,
                concat!(
                    "You must call this RPC with the \"more\" flag ( i.e. `--more` or ",
                    "`.more()` )"
                )
            ),
        }
    }
}