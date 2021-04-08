#![allow(nonstandard_style)]
// Generated from /home/stuart/Documents/juneau.rust/src/parsing/jasm/Jasm.g4 by ANTLR 4.8
use antlr_rust::tree::ParseTreeListener;
use antlr_rust::token_factory::CommonTokenFactory;
use super::jasmparser::*;

use std::any::Any;

pub trait JasmListener<'input> : ParseTreeListener<'input,JasmParserContextType>{

/**
 * Enter a parse tree produced by the {@code expressionStatement}
 * labeled alternative in {@link JasmParser#statement}.
 * @param ctx the parse tree
 */
fn enter_expressionStatement(&mut self, _ctx: &ExpressionStatementContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code expressionStatement}
 * labeled alternative in {@link JasmParser#statement}.
 * @param ctx the parse tree
 */
fn exit_expressionStatement(&mut self, _ctx: &ExpressionStatementContext<'input>) { }

/**
 * Enter a parse tree produced by the {@code ifStatement}
 * labeled alternative in {@link JasmParser#statement}.
 * @param ctx the parse tree
 */
fn enter_ifStatement(&mut self, _ctx: &IfStatementContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code ifStatement}
 * labeled alternative in {@link JasmParser#statement}.
 * @param ctx the parse tree
 */
fn exit_ifStatement(&mut self, _ctx: &IfStatementContext<'input>) { }

/**
 * Enter a parse tree produced by the {@code whileStatement}
 * labeled alternative in {@link JasmParser#statement}.
 * @param ctx the parse tree
 */
fn enter_whileStatement(&mut self, _ctx: &WhileStatementContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code whileStatement}
 * labeled alternative in {@link JasmParser#statement}.
 * @param ctx the parse tree
 */
fn exit_whileStatement(&mut self, _ctx: &WhileStatementContext<'input>) { }

/**
 * Enter a parse tree produced by the {@code functionStatement}
 * labeled alternative in {@link JasmParser#statement}.
 * @param ctx the parse tree
 */
fn enter_functionStatement(&mut self, _ctx: &FunctionStatementContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code functionStatement}
 * labeled alternative in {@link JasmParser#statement}.
 * @param ctx the parse tree
 */
fn exit_functionStatement(&mut self, _ctx: &FunctionStatementContext<'input>) { }

/**
 * Enter a parse tree produced by the {@code structDefinitionStatement}
 * labeled alternative in {@link JasmParser#statement}.
 * @param ctx the parse tree
 */
fn enter_structDefinitionStatement(&mut self, _ctx: &StructDefinitionStatementContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code structDefinitionStatement}
 * labeled alternative in {@link JasmParser#statement}.
 * @param ctx the parse tree
 */
fn exit_structDefinitionStatement(&mut self, _ctx: &StructDefinitionStatementContext<'input>) { }

/**
 * Enter a parse tree produced by the {@code declarationStatement}
 * labeled alternative in {@link JasmParser#statement}.
 * @param ctx the parse tree
 */
fn enter_declarationStatement(&mut self, _ctx: &DeclarationStatementContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code declarationStatement}
 * labeled alternative in {@link JasmParser#statement}.
 * @param ctx the parse tree
 */
fn exit_declarationStatement(&mut self, _ctx: &DeclarationStatementContext<'input>) { }

/**
 * Enter a parse tree produced by the {@code assignStatement}
 * labeled alternative in {@link JasmParser#statement}.
 * @param ctx the parse tree
 */
fn enter_assignStatement(&mut self, _ctx: &AssignStatementContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code assignStatement}
 * labeled alternative in {@link JasmParser#statement}.
 * @param ctx the parse tree
 */
fn exit_assignStatement(&mut self, _ctx: &AssignStatementContext<'input>) { }

/**
 * Enter a parse tree produced by the {@code returnStatement}
 * labeled alternative in {@link JasmParser#statement}.
 * @param ctx the parse tree
 */
fn enter_returnStatement(&mut self, _ctx: &ReturnStatementContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code returnStatement}
 * labeled alternative in {@link JasmParser#statement}.
 * @param ctx the parse tree
 */
fn exit_returnStatement(&mut self, _ctx: &ReturnStatementContext<'input>) { }

/**
 * Enter a parse tree produced by {@link JasmParser#if_statement}.
 * @param ctx the parse tree
 */
fn enter_if_statement(&mut self, _ctx: &If_statementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link JasmParser#if_statement}.
 * @param ctx the parse tree
 */
fn exit_if_statement(&mut self, _ctx: &If_statementContext<'input>) { }

/**
 * Enter a parse tree produced by {@link JasmParser#block}.
 * @param ctx the parse tree
 */
fn enter_block(&mut self, _ctx: &BlockContext<'input>) { }
/**
 * Exit a parse tree produced by {@link JasmParser#block}.
 * @param ctx the parse tree
 */
fn exit_block(&mut self, _ctx: &BlockContext<'input>) { }

/**
 * Enter a parse tree produced by {@link JasmParser#func}.
 * @param ctx the parse tree
 */
fn enter_func(&mut self, _ctx: &FuncContext<'input>) { }
/**
 * Exit a parse tree produced by {@link JasmParser#func}.
 * @param ctx the parse tree
 */
fn exit_func(&mut self, _ctx: &FuncContext<'input>) { }

/**
 * Enter a parse tree produced by the {@code binaryExpression}
 * labeled alternative in {@link JasmParser#expression}.
 * @param ctx the parse tree
 */
fn enter_binaryExpression(&mut self, _ctx: &BinaryExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code binaryExpression}
 * labeled alternative in {@link JasmParser#expression}.
 * @param ctx the parse tree
 */
fn exit_binaryExpression(&mut self, _ctx: &BinaryExpressionContext<'input>) { }

/**
 * Enter a parse tree produced by the {@code castExpression}
 * labeled alternative in {@link JasmParser#expression}.
 * @param ctx the parse tree
 */
fn enter_castExpression(&mut self, _ctx: &CastExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code castExpression}
 * labeled alternative in {@link JasmParser#expression}.
 * @param ctx the parse tree
 */
fn exit_castExpression(&mut self, _ctx: &CastExpressionContext<'input>) { }

/**
 * Enter a parse tree produced by the {@code arrayAccessExpression}
 * labeled alternative in {@link JasmParser#expression}.
 * @param ctx the parse tree
 */
fn enter_arrayAccessExpression(&mut self, _ctx: &ArrayAccessExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code arrayAccessExpression}
 * labeled alternative in {@link JasmParser#expression}.
 * @param ctx the parse tree
 */
fn exit_arrayAccessExpression(&mut self, _ctx: &ArrayAccessExpressionContext<'input>) { }

/**
 * Enter a parse tree produced by the {@code constantExpression}
 * labeled alternative in {@link JasmParser#expression}.
 * @param ctx the parse tree
 */
fn enter_constantExpression(&mut self, _ctx: &ConstantExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code constantExpression}
 * labeled alternative in {@link JasmParser#expression}.
 * @param ctx the parse tree
 */
fn exit_constantExpression(&mut self, _ctx: &ConstantExpressionContext<'input>) { }

/**
 * Enter a parse tree produced by the {@code referenceExpression}
 * labeled alternative in {@link JasmParser#expression}.
 * @param ctx the parse tree
 */
fn enter_referenceExpression(&mut self, _ctx: &ReferenceExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code referenceExpression}
 * labeled alternative in {@link JasmParser#expression}.
 * @param ctx the parse tree
 */
fn exit_referenceExpression(&mut self, _ctx: &ReferenceExpressionContext<'input>) { }

/**
 * Enter a parse tree produced by the {@code invocationExpression}
 * labeled alternative in {@link JasmParser#expression}.
 * @param ctx the parse tree
 */
fn enter_invocationExpression(&mut self, _ctx: &InvocationExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code invocationExpression}
 * labeled alternative in {@link JasmParser#expression}.
 * @param ctx the parse tree
 */
fn exit_invocationExpression(&mut self, _ctx: &InvocationExpressionContext<'input>) { }

/**
 * Enter a parse tree produced by the {@code structAccessExpression}
 * labeled alternative in {@link JasmParser#expression}.
 * @param ctx the parse tree
 */
fn enter_structAccessExpression(&mut self, _ctx: &StructAccessExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code structAccessExpression}
 * labeled alternative in {@link JasmParser#expression}.
 * @param ctx the parse tree
 */
fn exit_structAccessExpression(&mut self, _ctx: &StructAccessExpressionContext<'input>) { }

/**
 * Enter a parse tree produced by the {@code parenExpression}
 * labeled alternative in {@link JasmParser#expression}.
 * @param ctx the parse tree
 */
fn enter_parenExpression(&mut self, _ctx: &ParenExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code parenExpression}
 * labeled alternative in {@link JasmParser#expression}.
 * @param ctx the parse tree
 */
fn exit_parenExpression(&mut self, _ctx: &ParenExpressionContext<'input>) { }

/**
 * Enter a parse tree produced by the {@code unaryExpression}
 * labeled alternative in {@link JasmParser#expression}.
 * @param ctx the parse tree
 */
fn enter_unaryExpression(&mut self, _ctx: &UnaryExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code unaryExpression}
 * labeled alternative in {@link JasmParser#expression}.
 * @param ctx the parse tree
 */
fn exit_unaryExpression(&mut self, _ctx: &UnaryExpressionContext<'input>) { }

/**
 * Enter a parse tree produced by the {@code dereferenceExpression}
 * labeled alternative in {@link JasmParser#expression}.
 * @param ctx the parse tree
 */
fn enter_dereferenceExpression(&mut self, _ctx: &DereferenceExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code dereferenceExpression}
 * labeled alternative in {@link JasmParser#expression}.
 * @param ctx the parse tree
 */
fn exit_dereferenceExpression(&mut self, _ctx: &DereferenceExpressionContext<'input>) { }

/**
 * Enter a parse tree produced by the {@code variableExpression}
 * labeled alternative in {@link JasmParser#expression}.
 * @param ctx the parse tree
 */
fn enter_variableExpression(&mut self, _ctx: &VariableExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code variableExpression}
 * labeled alternative in {@link JasmParser#expression}.
 * @param ctx the parse tree
 */
fn exit_variableExpression(&mut self, _ctx: &VariableExpressionContext<'input>) { }

/**
 * Enter a parse tree produced by the {@code integerValue}
 * labeled alternative in {@link JasmParser#value}.
 * @param ctx the parse tree
 */
fn enter_integerValue(&mut self, _ctx: &IntegerValueContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code integerValue}
 * labeled alternative in {@link JasmParser#value}.
 * @param ctx the parse tree
 */
fn exit_integerValue(&mut self, _ctx: &IntegerValueContext<'input>) { }

/**
 * Enter a parse tree produced by the {@code floatValue}
 * labeled alternative in {@link JasmParser#value}.
 * @param ctx the parse tree
 */
fn enter_floatValue(&mut self, _ctx: &FloatValueContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code floatValue}
 * labeled alternative in {@link JasmParser#value}.
 * @param ctx the parse tree
 */
fn exit_floatValue(&mut self, _ctx: &FloatValueContext<'input>) { }

/**
 * Enter a parse tree produced by the {@code trueValue}
 * labeled alternative in {@link JasmParser#value}.
 * @param ctx the parse tree
 */
fn enter_trueValue(&mut self, _ctx: &TrueValueContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code trueValue}
 * labeled alternative in {@link JasmParser#value}.
 * @param ctx the parse tree
 */
fn exit_trueValue(&mut self, _ctx: &TrueValueContext<'input>) { }

/**
 * Enter a parse tree produced by the {@code falseValue}
 * labeled alternative in {@link JasmParser#value}.
 * @param ctx the parse tree
 */
fn enter_falseValue(&mut self, _ctx: &FalseValueContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code falseValue}
 * labeled alternative in {@link JasmParser#value}.
 * @param ctx the parse tree
 */
fn exit_falseValue(&mut self, _ctx: &FalseValueContext<'input>) { }

/**
 * Enter a parse tree produced by the {@code nullValue}
 * labeled alternative in {@link JasmParser#value}.
 * @param ctx the parse tree
 */
fn enter_nullValue(&mut self, _ctx: &NullValueContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code nullValue}
 * labeled alternative in {@link JasmParser#value}.
 * @param ctx the parse tree
 */
fn exit_nullValue(&mut self, _ctx: &NullValueContext<'input>) { }

/**
 * Enter a parse tree produced by the {@code stringValue}
 * labeled alternative in {@link JasmParser#value}.
 * @param ctx the parse tree
 */
fn enter_stringValue(&mut self, _ctx: &StringValueContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code stringValue}
 * labeled alternative in {@link JasmParser#value}.
 * @param ctx the parse tree
 */
fn exit_stringValue(&mut self, _ctx: &StringValueContext<'input>) { }

/**
 * Enter a parse tree produced by {@link JasmParser#parameter}.
 * @param ctx the parse tree
 */
fn enter_parameter(&mut self, _ctx: &ParameterContext<'input>) { }
/**
 * Exit a parse tree produced by {@link JasmParser#parameter}.
 * @param ctx the parse tree
 */
fn exit_parameter(&mut self, _ctx: &ParameterContext<'input>) { }

/**
 * Enter a parse tree produced by the {@code valueType}
 * labeled alternative in {@link JasmParser#typ}.
 * @param ctx the parse tree
 */
fn enter_valueType(&mut self, _ctx: &ValueTypeContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code valueType}
 * labeled alternative in {@link JasmParser#typ}.
 * @param ctx the parse tree
 */
fn exit_valueType(&mut self, _ctx: &ValueTypeContext<'input>) { }

/**
 * Enter a parse tree produced by the {@code pointerType}
 * labeled alternative in {@link JasmParser#typ}.
 * @param ctx the parse tree
 */
fn enter_pointerType(&mut self, _ctx: &PointerTypeContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code pointerType}
 * labeled alternative in {@link JasmParser#typ}.
 * @param ctx the parse tree
 */
fn exit_pointerType(&mut self, _ctx: &PointerTypeContext<'input>) { }

/**
 * Enter a parse tree produced by {@link JasmParser#memberAssign}.
 * @param ctx the parse tree
 */
fn enter_memberAssign(&mut self, _ctx: &MemberAssignContext<'input>) { }
/**
 * Exit a parse tree produced by {@link JasmParser#memberAssign}.
 * @param ctx the parse tree
 */
fn exit_memberAssign(&mut self, _ctx: &MemberAssignContext<'input>) { }

}
