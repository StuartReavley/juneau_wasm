use crate::core::Visitor;
use crate::semantic::{Module, Target, Functions};


pub trait ModuleBuildVisitor<S:Target, T:Target>: Sized + Visitor<S::Function, Option<T::Function>> {
    fn get_module_functions(&self) -> Option<Functions<T>> {
        None
    }

    fn visit(mut self, module:&Module<S>) -> Module<T> {
        let Module {main_id, library_ids, functions} = module;
        let mut built_functions = Functions::new();

        if let Some(id) = *main_id {
            built_functions.insert(id, <Self as Visitor<_, _>>::visit(&mut self, functions.get(id, &"module_visit".into())).unwrap());
        }

        for &id in library_ids {
            built_functions.insert(id, <Self as Visitor<_, _>>::visit(&mut self, functions.get(id, &"module_visit".into())).unwrap());
        }
        
        let main_id = *main_id;
        let library_ids = library_ids.to_owned();
        let functions = self.get_module_functions().unwrap_or(built_functions);
        Module {main_id, library_ids, functions}
    }
}