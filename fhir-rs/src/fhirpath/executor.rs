use super::*;
use crate::prelude::*;

use std::fmt::Debug;

pub trait Executor: Base + Debug {
    fn as_collection(&self) -> Collection;
    fn as_collection2(&self) -> PathResponse {
        PathResponse::Collection(self.as_collection())
    }

    fn as_response(&self, _comp: &PathComponent) -> PathResponse {
        PathResponse::Collection(self.as_collection())
    }

    fn exec(&self, comp: &PathComponent) -> Result<PathResponse> {
        Err(FhirError::Message(format!("String: 基础类型不支持的函数:{:?}", comp)))
    }

    fn eval(&self, expr: &PathExpression) -> Result<PathResponse> {
        let mut resp = self.as_collection2();
        while let (Some(comp), _) = expr.next() {
            resp = resp.exec(&comp)?;
            println!("Response: {:?}", &resp)
        }
        
        Ok(resp)
    }

    fn assert(&self, expr: &PathExpression) -> Result<bool> {
        if expr.response() != FunctionResponse::Bool {
            return Err(FhirError::error("该表达式不是一个有效的路径表达式，最后返回值不是Bool"));
        }

        match &expr.root {
            Root::Absolute(name) => {
                if name == &self.type_name() {
                    expr.eat();
                } else {
                    return Err(FhirError::Message(format!("根元素[{}]与当前数据类型不匹配", &name)))
                }
            },
            _ => {},
        }

        let resp = self.eval(expr)?;

        match resp {
            PathResponse::Bool(bool) => Ok(bool),
            _ => Err(FhirError::error("断言的结果值应为Boolean类型"))
        }
    }

    fn path(&self, expr: &mut PathExpression) -> Result<Collection> {
        if expr.response() != FunctionResponse::Collection {
            return Err(FhirError::error("该表达式不是一个有效的路径表达式，最后返回值不是Collection"));
        }

        match &expr.root {
            Root::Absolute(name) => {
                if name == &self.type_name() {
                    expr.eat();
                } else {
                    return Err(FhirError::Message(format!("根元素[{}]与当前数据类型不匹配", &name)))
                }
            },
            _ => {},
        }

        let resp = self.eval(expr)?;

        match resp {
            PathResponse::Collection(collection) => Ok(collection),
            _ => unreachable!(),
        }
    }
}

macro_rules! impl_executor {
    (
        $($ty:ident,)+
    ) => {
        $(
            impl Executor for $ty {
                fn exec(&self, _comp: &PathComponent) -> Result<PathResponse> {
                    Ok(self.as_collection2())
                }
            
                fn as_collection(&self) -> Collection {
                    Collection::with(Box::new(self.clone()))
                }
            }
        )+
    }
}

impl_executor!(usize, isize, i64, f64, bool, Instant, DateTime, Time, Date, String,);

impl<T: Executor> Executor for Box<T> {
    fn as_collection(&self) -> Collection {
        self.as_ref().as_collection()
    }
}

impl<T: Executor> Executor for Option<T> {
    fn exec(&self, comp: &PathComponent) -> Result<PathResponse> {
        match self {
            Some(value) => Ok(value.as_response(comp)),
            None => Ok(PathResponse::Collection(Collection::new())),
        }
    }

    fn as_collection(&self) -> Collection {
        match self {
            Some(value) => value.as_collection(),
            None => Collection::new(),
        }
    }
}

impl<T: Executor> Executor for Vec<T> {
    fn exec(&self, comp: &PathComponent) -> Result<PathResponse> {
        unreachable!()
    }

    fn as_response(&self, comp: &PathComponent) -> PathResponse {
        match comp {
            PathComponent::Path(path) => {
                match path.index {
                    Some(index) => {
                        match self.get(index) {
                            Some(val) => val.as_collection2(),
                            None => PathResponse::Collection(Collection::new()),
                        }
                    },
                    None => self.as_collection2(),
                }
            },
            _ => unreachable!(),
        }
    }

    fn as_collection(&self) -> Collection {
        let mut vv = Collection::new();
        for item in self {
            vv.combine(item.as_collection())
        };
        vv
    }
}

