use std::path::PathBuf;

pub use error_chain::ChainedError;
pub use lalrpop_util::ParseError;

use sql::lexer;
use connection::{ConnectionError, ConnectionErrorKind};

error_chain! {
    types {
        PsqlpackError, PsqlpackErrorKind, PsqlpackResultExt, PsqlpackResult;
    }
    links {
        Connection(ConnectionError, ConnectionErrorKind);
    }
    errors {
        ProjectReadError(path: PathBuf) {
            description("Couldn't read project file")
            display("Couldn't read project file: {}", path.as_path().display())
        }
        ProjectParseError(path: PathBuf) {
            description("Couldn't parse project file")
            display("Couldn't parse project file: {}", path.as_path().display())
        }
        InvalidScriptPath(path: String) {
            description("Invalid script path in project file")
            display("Invalid script path in project file: {}", path)
        }
        PublishProfileReadError(path: PathBuf) {
            description("Couldn't read publish profile file")
            display("Couldn't read publish profile file: {}", path.as_path().display())
        }
        PublishProfileParseError(path: PathBuf) {
            description("Couldn't parse publish profile file")
            display("Couldn't parse publish profile file: {}", path.as_path().display())
        }
        PackageReadError(path: PathBuf) {
            description("Couldn't read package file")
            display("Couldn't read package file: {}", path.as_path().display())
        }
        PackageUnarchiveError(path: PathBuf) {
            description("Couldn't unarchive package file")
            display("Couldn't unarchive package file: {}", path.as_path().display())
        }
        PackageInternalReadError(file_name: String) {
            description("Couldn't read part of the package file")
            display("Couldn't read part of the package file: {}", file_name)
        }
        PackageQueryExtensionsError {
            description("Couldn't query extensions")
        }
        PackageQuerySchemasError {
            description("Couldn't query schemas")
        }
        PackageQueryTypesError {
            description("Couldn't query types")
        }
        PackageQueryFunctionsError {
            description("Couldn't query functions")
        }
        PackageQueryTablesError {
            description("Couldn't query tables")
        }
        PackageFunctionArgsInspectError(args: String) {
            description("Couldn't inspect function args")
            display("Couldn't inspect function args: {}", args)
        }
        PackageFunctionReturnTypeInspectError(return_type: String) {
            description("Couldn't inspect function return type")
            display("Couldn't inspect function return type: {}", return_type)
        }
        IOError(file: String, message: String) {
            description("IO error when reading a file")
            display("IO error when reading {}: {}", file, message)
        }
        LexicalError(line: String, line_number: usize, start: usize, end: usize) {
            description("Lexical error encountered")
            display("Lexical error encountered on line {}:\n  {}\n  {}{}",
                line_number, line, " ".repeat(*start), "^".repeat(end - start))
        }
        SyntaxError(file: String, line: String, line_number: usize, start: usize, end: usize) {
            description("SQL syntax error encountered")
            display(
                "SQL syntax error encountered in {} on line {}:\n  {}\n  {}{}",
                file, line_number, line, " ".repeat(*start), "^".repeat(end - start))
        }
        ParseError(file: String, errors: Vec<ParseError<(), lexer::Token, ()>>) {
            description("Parser error")
            display("Parser errors in {}:\n{}", file, ParseErrorsFormatter(errors))
        }
        InlineParseError(error: ParseError<(), lexer::Token, ()>) {
            description("Parser error")
            display("Parser error: {}", ParseErrorFormatter(error))
        }
        GenerationError(message: String) {
            description("Error generating package")
            display("Error generating package: {}", message)
        }
        FormatError(file: String, message: String) {
            description("Format error when reading a file")
            display("Format error when reading {}: {}", file, message)
        }
        DatabaseError(message: String) {
            description("Database error")
            display("Database error: {}", message)
        }
        DatabaseExecuteError(query: String) {
            description("Database error executing query")
            display("Database error executing: {}", query)
        }
        DatabaseConnectionFinishError {
            description("Database connection couldn't finish")
            display("Database connection couldn't finish")
        }
        ProjectError(message: String) {
            description("Project format error")
            display("Project format error: {}", message)
        }
        MultipleErrors(errors: Vec<PsqlpackError>) {
            description("Multiple errors")
            display("Multiple errors:\n{}", MultipleErrorFormatter(errors))
        }
    }
}

use std::fmt::{Display, Formatter, Result};

fn write_err(f: &mut Formatter, error: &ParseError<(), lexer::Token, ()>) -> Result {
    match *error {
        ParseError::InvalidToken { .. } => write!(f, "Invalid token"),
        ParseError::UnrecognizedToken {
            ref token,
            ref expected,
        } => {
            match *token {
                Some(ref x) => writeln!(f, "Unexpected {:?}", x.1),
                _ => writeln!(f, "Unexpected end of file"),
            }?;
            write!(f, "   Expected one of:\n   {}", expected.join(", "))
        }
        ParseError::ExtraToken { ref token } => {
            write!(f, "Extra token detected: {:?}", token)
        }
        ParseError::User { ref error } => write!(f, "{:?}", error),
    }
}

struct ParseErrorsFormatter<'fmt>(&'fmt Vec<ParseError<(), lexer::Token, ()>>);

impl<'fmt> Display for ParseErrorsFormatter<'fmt> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        for (i, error) in self.0.iter().enumerate() {
            write!(f, "{}: ", i, )?;
            write_err(f, error)?;
        }
        Ok(())
    }
}

struct ParseErrorFormatter<'fmt>(&'fmt ParseError<(), lexer::Token, ()>);

impl<'fmt> Display for ParseErrorFormatter<'fmt> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write_err(f, self.0)
    }
}

struct MultipleErrorFormatter<'fmt>(&'fmt Vec<PsqlpackError>);

impl<'fmt> Display for MultipleErrorFormatter<'fmt> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        for (i, error) in self.0.iter().enumerate() {
            write!(f, "--- Error {} ---\n{}", i, error)?;
        }
        Ok(())
    }
}
