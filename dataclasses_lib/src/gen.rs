use codegen::Scope;

use super::def::Dataclass;

trait Codegen {
    fn generate(self, scope: &mut Scope);
}

impl Codegen for Dataclass {
    fn generate(self, scope: &mut Scope) {
        let s = scope.new_struct(&self.name);
        s.doc("nice").field(name, ty);
    }
}