use super::{
    code_block::{parse_code_block, CodeBlock},
    types::parse_type,
    utility::*,
    DataType,
};

/// A single function parameter
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Parameter {
    pub name: String,
    pub data_type: DataType,
}

/// Definition of a function
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct FunctionDef {
    pub identifier: String,
    pub params: Vec<Parameter>,
    pub code_block: CodeBlock,
    pub data_type: DataType,
}

/// parameter
///     = IDENTIFIER, TYPE_SIGNATURE, type
///     ;
fn parse_parameter(p: &mut Parser) -> OptRes<Parameter> {
    if let Some(name) = p.identifier()? {
        if !p.operator(Op::Colon)? {
            p.warn(WarnVar::MissingColon);
        }
        if let Some(data_type) = parse_type(p)? {
            Ok(Some(Parameter { name, data_type }))
        } else {
            p.error(ErroVar::FunctionParameterMissingType)
        }
    } else {
        Ok(None)
    }
}

/// parameters
///     = [parameter, {SPLIT, parameter}]
///     ;
fn parse_parameters(p: &mut Parser) -> Res<Vec<Parameter>> {
    let mut params = vec![];
    while let Some(param) = parse_parameter(p)? {
        params.push(param);
    }
    Ok(params)
}

/// function_definition
///     = KW_FN, OPEN_BRACKET, parameters, CLOSE_BRACKET, [RETURN_SIGNATURE, type], code_block
///     ;
pub fn parse_function_def(p: &mut Parser) -> OptRes<FunctionDef> {
    if !p.keyword(Kw::Fn)? {
        return Ok(None);
    }
    if let Some(identifier) = p.identifier()? {
        if !p.operator(Op::OpenRoundBracket)? {
            p.warn(WarnVar::MissingOpeningRoundBracket);
        }
        let params = parse_parameters(p)?;
        if !p.operator(Op::CloseRoundBracket)? {
            p.warn(WarnVar::MissingClosingRoundBracket);
        }
        let data_type = if p.operator(Op::Arrow)? {
            if let Some(data_type) = parse_type(p)? {
                data_type
            } else {
                return p.error(ErroVar::FunctionMissingReturnType);
            }
        } else {
            DataType::None
        };
        if let Some(code_block) = parse_code_block(p)? {
            Ok(Some(FunctionDef {
                identifier,
                params,
                code_block,
                data_type,
            }))
        } else {
            p.error(ErroVar::FunctionMissingBody)
        }
    } else {
        p.error(ErroVar::FunctionMissingIdentifier)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {}
}
