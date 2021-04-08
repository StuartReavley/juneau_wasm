use std::cell::RefCell;
use std::rc::Rc;
use std::collections::HashMap;
use crate::{core::{Id, IdContext, IdProvider}, parsing::jasm::gen::JasmParserContext};
use crate::semantic::{Name, Parameter, Variable};
use crate::semantic::jasm::{Jasm, JasmType, Struct};
use crate::parsing::{ParseVisitor, Symbols, Scope};


pub struct JasmParseVisitor {
    ids: Rc<RefCell<IdProvider>>,
    pub scope: Scope<Jasm>,
    structs: HashMap<Id, Struct<Parameter<Jasm>>>
}

impl ParseVisitor for JasmParseVisitor {}

impl JasmParseVisitor {
    pub fn new(ids:Rc<RefCell<IdProvider>>, symbols:Symbols<Jasm>) -> Self {
        let structs = HashMap::new();
        let scope = Scope::new(symbols);
        Self {ids, scope, structs}
    }

    pub fn new_symbol(&mut self, name:&Name, typ:JasmType) -> Id {
        let id = self.new_id();
        self.scope.insert(Variable::new(id, name, &typ));
        id
    }

    pub fn new_struct(&mut self, name:Name, parameters:Vec<Parameter<Jasm>>) -> Struct<Parameter<Jasm>> {
        let id = self.new_id();
        self.scope.insert(Variable::new(id, &name, &JasmType::Struct(id, name.to_owned())));
        let struct_definition = Struct {id, name, parameters};
        self.structs.insert(id, struct_definition.to_owned());
        struct_definition
    }

    pub fn get_parameter(&self, struct_id:Id, parameter_name:&Name) -> &Parameter<Jasm> {
        let struct_definition = self.structs.get(&struct_id).expect(&format!("cannot find struct [{:?}]", struct_id));
        let parameters = &struct_definition.parameters;
        parameters
            .iter()
            .find(|p| p.name == parameter_name)
            .expect(&format!("cannot find parameter [{:?}]", parameter_name))
    }
}

impl IdContext for JasmParseVisitor {
    fn new_id(&mut self) -> Id {
        self.ids.new_id()
    }
}