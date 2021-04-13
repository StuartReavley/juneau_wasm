use std::ffi::c_void;
use std::rc::Rc;
use std::mem::transmute;
use crate::core::c::CSemiBox;
use crate::target::llvm::{native_reference, Function};
use super::{ExecutionEngine, JitEngine};


native_reference! {
    ExecutableFunction = c_void
}


#[derive(Clone)]
pub struct ExecutableFunction2<'l> {
    engine: Rc<CSemiBox<'l, JitEngine>>,
    function: &'l Function
}

impl<'l> ExecutableFunction2<'l> {
    pub fn new(engine: &Rc<CSemiBox<'l, JitEngine>>, function:&'l Function) -> Self {
        let engine = engine.to_owned();
        Self {engine, function}
    }

    pub fn call0 <                                             R>(&self                                                                           ) -> R {let ff:extern "C" fn(                                           ) -> R = unsafe{transmute(self.get_address())};ff(                                           )}
    pub fn call1 <A,                                           R>(&self, a:A                                                                      ) -> R {let ff:extern "C" fn(A                                          ) -> R = unsafe{transmute(self.get_address())};ff(a                                          )}
    pub fn call2 <A, B,                                        R>(&self, a:A, b:B                                                                 ) -> R {let ff:extern "C" fn(A, B                                       ) -> R = unsafe{transmute(self.get_address())};ff(a, b                                       )}
    pub fn call3 <A, B, C,                                     R>(&self, a:A, b:B, c:C                                                            ) -> R {let ff:extern "C" fn(A, B, C                                    ) -> R = unsafe{transmute(self.get_address())};ff(a, b, c                                    )}
    pub fn call4 <A, B, C, D,                                  R>(&self, a:A, b:B, c:C, d:D                                                       ) -> R {let ff:extern "C" fn(A, B, C, D                                 ) -> R = unsafe{transmute(self.get_address())};ff(a, b, c, d                                 )}
    pub fn call5 <A, B, C, D, E,                               R>(&self, a:A, b:B, c:C, d:D, e:E                                                  ) -> R {let ff:extern "C" fn(A, B, C, D, E                              ) -> R = unsafe{transmute(self.get_address())};ff(a, b, c, d, e                              )}
    pub fn call6 <A, B, C, D, E, F,                            R>(&self, a:A, b:B, c:C, d:D, e:E, f:F                                             ) -> R {let ff:extern "C" fn(A, B, C, D, E, F                           ) -> R = unsafe{transmute(self.get_address())};ff(a, b, c, d, e, f                           )}
    pub fn call7 <A, B, C, D, E, F, G,                         R>(&self, a:A, b:B, c:C, d:D, e:E, f:F, g:G                                        ) -> R {let ff:extern "C" fn(A, B, C, D, E, F, G                        ) -> R = unsafe{transmute(self.get_address())};ff(a, b, c, d, e, f, g                        )}
    pub fn call8 <A, B, C, D, E, F, G, H,                      R>(&self, a:A, b:B, c:C, d:D, e:E, f:F, g:G, h:H                                   ) -> R {let ff:extern "C" fn(A, B, C, D, E, F, G, H,                    ) -> R = unsafe{transmute(self.get_address())};ff(a, b, c, d, e, f, g, h,                    )}
    pub fn call10<A, B, C, D, E, F, G, H, I, J,                R>(&self, a:A, b:B, c:C, d:D, e:E, f:F, g:G, h:H, i:I, j:J                         ) -> R {let ff:extern "C" fn(A, B, C, D, E, F, G, H, I, J               ) -> R = unsafe{transmute(self.get_address())};ff(a, b, c, d, e, f, g, h, i, j               )}
    pub fn call13<A, B, C, D, E, F, G, H, I, J, K, L, M,       R>(&self, a:A, b:B, c:C, d:D, e:E, f:F, g:G, h:H, i:I, j:J, k:K, l:L, m:M          ) -> R {let ff:extern "C" fn(A, B, C, D, E, F, G, H, I, J, K, L, M      ) -> R = unsafe{transmute(self.get_address())};ff(a, b, c, d, e, f, g, h, i, j, k, l, m      )}
    pub fn call15<A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, R>(&self, a:A, b:B, c:C, d:D, e:E, f:F, g:G, h:H, i:I, j:J, k:K, l:L, m:M, n:N, o:O) -> R {let ff:extern "C" fn(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O) -> R = unsafe{transmute(self.get_address())};ff(a, b, c, d, e, f, g, h, i, j, k, l, m, n, o)}

    fn get_address(&self) -> &ExecutableFunction {
        self.engine.get_function_address(self.function).unwrap()
    }
}

impl<'l> std::fmt::Debug for ExecutableFunction2<'l> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("[ExecutableFunction2]")
    }
}

