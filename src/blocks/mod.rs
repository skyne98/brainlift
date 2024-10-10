use cranelift::{
    codegen::ir::{Function, UserFuncName},
    prelude::{isa::CallConv, AbiParam, Signature, Type},
};

pub struct BrainFunctionBuilder {
    params: Vec<Type>,
    returns: Vec<Type>,
}

impl BrainFunctionBuilder {
    pub fn new() -> Self {
        Self {
            params: Vec::new(),
            returns: Vec::new(),
        }
    }

    pub fn with_param(mut self, ty: Type) -> Self {
        self.params.push(ty);
        self
    }

    pub fn with_return(mut self, ty: Type) -> Self {
        self.returns.push(ty);
        self
    }

    pub fn build(self) -> BrainFunction {
        let mut sig = Signature::new(CallConv::SystemV);

        for param in self.params {
            sig.params.push(AbiParam::new(param));
        }
        for ret in self.returns {
            sig.returns.push(AbiParam::new(ret));
        }

        let func = Function::with_name_signature(UserFuncName::default(), sig);
        BrainFunction { function: func }
    }
}

pub struct BrainFunction {
    function: Function,
}

impl BrainFunction {
    pub fn cranelift(&self) -> &Function {
        &self.function
    }
}
