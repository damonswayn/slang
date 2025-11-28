use std::fmt::{self, Display, Formatter};

#[derive(Debug, Clone)]
pub struct Program {
    pub statements: Vec<Statement>,
}

impl Program {
    pub fn new() -> Self {
        Program { statements: Vec::new() }
    }
}

impl Display for Program {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for stmt in &self.statements {
            write!(f, "{}", stmt)?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    Let(LetStatement),
    Return(ReturnStatement),
    Expression(ExpressionStatement),
    While(WhileStatement),
    For(ForStatement),
    Function(FunctionStatement),
}

impl Display for Statement {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Statement::Let(ls) => write!(f, "{}", ls),
            Statement::Return(rs) => write!(f, "{}", rs),
            Statement::While(ws) => write!(f, "{}", ws),
            Statement::For(fs) => write!(f, "{}", fs),
            Statement::Expression(es) => write!(f, "{}", es),
            Statement::Function(fs) => write!(f, "{}", fs),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct LetStatement {
    pub name: Identifier,
    pub value: Expression,
}

impl Display for LetStatement {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "let {} = {};", self.name, self.value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ExpressionStatement {
    pub expression: Expression,
}

impl Display for ExpressionStatement {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.expression)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct BlockStatement {
    pub statements: Vec<Statement>,
}

impl Display for BlockStatement {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for stmt in &self.statements {
            write!(f, "{}", stmt)?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ReturnStatement {
    pub return_value: Expression,
}

impl Display for ReturnStatement {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "return {};", self.return_value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct WhileStatement {
    pub condition: Expression,
    pub body: BlockStatement,
}

impl Display for WhileStatement {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "while ({}) {{", self.condition)?;
        write!(f, "{}", self.body)?;
        write!(f, "}}")
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ForStatement {
    pub init: Option<Box<Statement>>,
    pub condition: Option<Expression>,
    pub post: Option<Box<Statement>>,
    pub body: BlockStatement,
}

impl Display for ForStatement {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "for (")?;
        if let Some(init) = &self.init { write!(f, "{}", init)?; }
        write!(f, "; ")?;
        if let Some(cond) = &self.condition { write!(f, "{}", cond)?; }
        write!(f, "; ")?;
        if let Some(post) = &self.post { write!(f, "{}", post)?; }
        write!(f, ") {}", self.body)?;
        Ok(())
    }
}

// ---------- Expressions ----------

#[derive(Debug, Clone, PartialEq)]
pub struct Identifier {
    pub value: String,
}

impl Display for Identifier {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Identifier(Identifier),
    IntegerLiteral(IntegerLiteral),
    BooleanLiteral(BooleanLiteral),
    FloatLiteral(FloatLiteral),
    StringLiteral(StringLiteral),
    Infix(InfixExpression),
    If(Box<IfExpression>),
    Prefix(Box<PrefixExpression>),
    Postfix(Box<PostfixExpression>),
    FunctionLiteral(FunctionLiteral),
    CallExpression(Box<CallExpression>),
    ArrayLiteral(ArrayLiteral),
    IndexExpression(Box<IndexExpression>),
    ObjectLiteral(ObjectLiteral),
    PropertyAccess(Box<PropertyAccess>),
}

impl Display for Expression {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Expression::Identifier(ident) => write!(f, "{}", ident),
            Expression::IntegerLiteral(il) => write!(f, "{}", il),
            Expression::BooleanLiteral(bl) => write!(f, "{}", bl),
            Expression::FloatLiteral(fl) => write!(f, "{}", fl),
            Expression::StringLiteral(sl) => write!(f, "{}", sl),
            Expression::Infix(infix) => write!(f, "{}", infix),
            Expression::If(ifexpr) => write!(f, "{}", ifexpr),
            Expression::Prefix(prefix) => write!(f, "{}", prefix),
            Expression::Postfix(postfix) => write!(f, "{}", postfix),
            Expression::FunctionLiteral(fl) => write!(f, "{}", fl),
            Expression::CallExpression(call) => write!(f, "{}", call),
            Expression::ArrayLiteral(al) => write!(f, "{}", al),
            Expression::IndexExpression(ie) => write!(f, "{}", ie),
            Expression::ObjectLiteral(ol) => write!(f, "{}", ol),
            Expression::PropertyAccess(pa) => write!(f, "{}", pa),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct IntegerLiteral {
    pub value: i64,
}

impl Display for IntegerLiteral {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct FloatLiteral {
    pub value: f64,
}

impl Display for FloatLiteral {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct BooleanLiteral {
    pub value: bool,
}

impl Display for BooleanLiteral {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct StringLiteral {
    pub value: String,
}

impl Display for StringLiteral {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        // You can print without quotes or with; I’ll include quotes:
        write!(f, "\"{}\"", self.value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ArrayLiteral {
    pub elements: Vec<Expression>,
}

impl Display for ArrayLiteral {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "[")?;
        for (i, e) in self.elements.iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}", e)?;
        }
        write!(f, "]")
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct IndexExpression {
    pub left: Box<Expression>,
    pub index: Box<Expression>,
}

impl Display for IndexExpression {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}[{}]", self.left, self.index)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ObjectLiteral {
    /// Properties in insertion order: `name: expr`
    pub properties: Vec<(Identifier, Expression)>,
}

impl Display for ObjectLiteral {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{{")?;
        for (i, (name, value)) in self.properties.iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}: {}", name, value)?;
        }
        write!(f, "}}")
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct PropertyAccess {
    pub object: Box<Expression>,
    pub property: Identifier,
}

impl Display for PropertyAccess {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{}", self.object, self.property)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum InfixOp {
    Assign,
    And,
    Or,
    Equals,
    NotEquals,
    LessThan,
    LessEqual,
    GreaterThan,
    GreaterEqual,
    Plus,
    Minus,
    Multiply,
    Divide,
    Modulo,
}

impl Display for InfixOp {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let s = match self {
            InfixOp::Assign => "=",
            InfixOp::And => "&&",
            InfixOp::Or => "||",
            InfixOp::Equals => "==",
            InfixOp::NotEquals => "!=",
            InfixOp::LessThan => "<",
            InfixOp::LessEqual => "<=",
            InfixOp::GreaterThan => ">",
            InfixOp::GreaterEqual => ">=",
            InfixOp::Plus => "+",
            InfixOp::Minus => "-",
            InfixOp::Multiply => "*",
            InfixOp::Divide => "/",
            InfixOp::Modulo => "%",
        };
        write!(f, "{}", s)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct InfixExpression {
    pub left: Box<Expression>,
    pub operator: InfixOp,
    pub right: Box<Expression>,
}

impl Display for InfixExpression {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "({} {} {})", *self.left, self.operator, *self.right)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct IfExpression {
    pub condition: Box<Expression>,
    pub consequence: BlockStatement,
    pub alternative: Option<BlockStatement>,
}

impl Display for IfExpression {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "if ({}) {{", *self.condition)?;
        write!(f, "{}", self.consequence)?;
        write!(f, "}}")?;

        if let Some(alt) = &self.alternative {
            write!(f, " else {{")?;
            write!(f, "{}", alt)?;
            write!(f, "}}")?;
        }

        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum PrefixOp {
    Not,
    Negate,
    PreIncrement,
    PreDecrement,
}

impl Display for PrefixOp {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let s = match self {
            PrefixOp::Not => "!",
            PrefixOp::Negate => "-",
            PrefixOp::PreIncrement => "++",
            PrefixOp::PreDecrement => "--",
        };
        write!(f, "{}", s)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct PrefixExpression {
    pub operator: PrefixOp,
    pub right: Box<Expression>,
}

impl Display for PrefixExpression {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "({}{})", self.operator, self.right)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum PostfixOp {
    Increment,
    Decrement,
}

impl Display for PostfixOp {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let s = match self {
            PostfixOp::Increment => "++",
            PostfixOp::Decrement => "--",
        };
        write!(f, "{}", s)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct PostfixExpression {
    pub left: Box<Expression>,
    pub operator: PostfixOp,
}

impl Display for PostfixExpression {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "({}{})", self.left, self.operator)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionLiteral {
    pub params: Vec<Identifier>,
    pub body: BlockStatement,
}

impl Display for FunctionLiteral {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "fn(")?;
        for (i, p) in self.params.iter().enumerate() {
            if i > 0 { write!(f, ", ")?; }
            write!(f, "{}", p)?;
        }
        write!(f, ") {{")?;
        write!(f, "{}", self.body)?;
        write!(f, "}}")
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionStatement {
    pub name: Identifier,
    pub literal: FunctionLiteral,
}

impl Display for FunctionStatement {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "fn(")?;
        for (i, p) in self.literal.params.iter().enumerate() {
            if i > 0 { write!(f, ", ")?; }
            write!(f, "{}", p)?;
        }
        write!(f, ") {{")?;
        write!(f, "{}", self.literal.body)?;
        write!(f, "}}")
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct CallExpression {
    pub function: Box<Expression>,    // identifier or fn literal
    pub arguments: Vec<Expression>,
}

impl Display for CallExpression {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}(", *self.function)?;
        for (i, arg) in self.arguments.iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}", arg)?;
        }
        write!(f, ")")
    }
}

// ---------- Tests ----------

#[cfg(test)]
mod tests {
    use super::{
        Expression, Identifier, InfixExpression, InfixOp, IntegerLiteral, LetStatement, Program,
        Statement,
    };

    #[test]
    fn program_display_renders_let() {
        let stmt = Statement::Let(LetStatement {
            name: Identifier { value: "x".to_string() },
            value: Expression::IntegerLiteral(IntegerLiteral { value: 5 }),
        });

        let mut program = Program::new();
        program.statements.push(stmt);

        assert_eq!(program.to_string(), "let x = 5;");
    }

    #[test]
    fn infix_display_renders_parens() {
        let expr = Expression::Infix(InfixExpression {
            left: Box::new(Expression::IntegerLiteral(IntegerLiteral { value: 1 })),
            operator: InfixOp::Plus,
            right: Box::new(Expression::Infix(InfixExpression {
                left: Box::new(Expression::IntegerLiteral(IntegerLiteral { value: 2 })),
                operator: InfixOp::Multiply,
                right: Box::new(Expression::IntegerLiteral(IntegerLiteral { value: 3 })),
            })),
        });

        assert_eq!(expr.to_string(), "(1 + (2 * 3))");
    }
}