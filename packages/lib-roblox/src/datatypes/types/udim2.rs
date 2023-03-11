use core::fmt;
use std::ops;

use glam::Vec2;
use mlua::prelude::*;
use rbx_dom_weak::types::UDim2 as RbxUDim2;

use super::{super::*, UDim};

/**
    An implementation of the [UDim2](https://create.roblox.com/docs/reference/engine/datatypes/UDim2) Roblox datatype.

    This implements all documented properties, methods & constructors of the UDim2 class as of March 2023.
*/
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct UDim2 {
    pub(crate) x: UDim,
    pub(crate) y: UDim,
}

impl UDim2 {
    pub(crate) fn make_table(lua: &Lua, datatype_table: &LuaTable) -> LuaResult<()> {
        datatype_table.set(
            "fromScale",
            lua.create_function(|_, (x, y): (Option<f32>, Option<f32>)| {
                Ok(UDim2 {
                    x: UDim {
                        scale: x.unwrap_or_default(),
                        offset: 0,
                    },
                    y: UDim {
                        scale: y.unwrap_or_default(),
                        offset: 0,
                    },
                })
            })?,
        )?;
        datatype_table.set(
            "fromOffset",
            lua.create_function(|_, (x, y): (Option<i32>, Option<i32>)| {
                Ok(UDim2 {
                    x: UDim {
                        scale: 0f32,
                        offset: x.unwrap_or_default(),
                    },
                    y: UDim {
                        scale: 0f32,
                        offset: y.unwrap_or_default(),
                    },
                })
            })?,
        )?;
        type ArgsUDims = (Option<UDim>, Option<UDim>);
        type ArgsNums = (Option<f32>, Option<i32>, Option<f32>, Option<i32>);
        datatype_table.set(
            "new",
            lua.create_function(|lua, args: LuaMultiValue| {
                if let Ok((x, y)) = ArgsUDims::from_lua_multi(args.clone(), lua) {
                    Ok(UDim2 {
                        x: x.unwrap_or_default(),
                        y: y.unwrap_or_default(),
                    })
                } else if let Ok((sx, ox, sy, oy)) = ArgsNums::from_lua_multi(args, lua) {
                    Ok(UDim2 {
                        x: UDim {
                            scale: sx.unwrap_or_default(),
                            offset: ox.unwrap_or_default(),
                        },
                        y: UDim {
                            scale: sy.unwrap_or_default(),
                            offset: oy.unwrap_or_default(),
                        },
                    })
                } else {
                    // TODO: Better error message here using arg types
                    Err(LuaError::RuntimeError(
                        "Invalid arguments to constructor".to_string(),
                    ))
                }
            })?,
        )
    }
}

impl fmt::Display for UDim2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}, {}", self.x, self.y)
    }
}

impl ops::Neg for UDim2 {
    type Output = Self;
    fn neg(self) -> Self::Output {
        UDim2 {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl ops::Add for UDim2 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        UDim2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl ops::Sub for UDim2 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        UDim2 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl LuaUserData for UDim2 {
    fn add_fields<'lua, F: LuaUserDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_field_method_get("X", |_, this| Ok(this.x));
        fields.add_field_method_get("Y", |_, this| Ok(this.y));
        fields.add_field_method_get("Width", |_, this| Ok(this.x));
        fields.add_field_method_get("Height", |_, this| Ok(this.y));
    }

    fn add_methods<'lua, M: LuaUserDataMethods<'lua, Self>>(methods: &mut M) {
        // Methods
        methods.add_method("Lerp", |_, this, (rhs, alpha): (UDim2, f32)| {
            let this_vec_x = Vec2::new(this.x.scale, this.x.offset as f32);
            let this_vec_y = Vec2::new(this.y.scale, this.y.offset as f32);
            let rhs_vec_x = Vec2::new(rhs.x.scale, rhs.x.offset as f32);
            let rhs_vec_y = Vec2::new(rhs.y.scale, rhs.y.offset as f32);
            let result_x = this_vec_x.lerp(rhs_vec_x, alpha);
            let result_y = this_vec_y.lerp(rhs_vec_y, alpha);
            Ok(UDim2 {
                x: UDim {
                    scale: result_x.x,
                    offset: result_x.y.clamp(i32::MIN as f32, i32::MAX as f32).round() as i32,
                },
                y: UDim {
                    scale: result_y.x,
                    offset: result_y.y.clamp(i32::MIN as f32, i32::MAX as f32).round() as i32,
                },
            })
        });
        // Metamethods
        methods.add_meta_method(LuaMetaMethod::Eq, userdata_impl_eq);
        methods.add_meta_method(LuaMetaMethod::ToString, userdata_impl_to_string);
        methods.add_meta_method(LuaMetaMethod::Unm, |_, this, ()| Ok(-*this));
        methods.add_meta_method(LuaMetaMethod::Add, |_, this, rhs: UDim2| Ok(*this + rhs));
        methods.add_meta_method(LuaMetaMethod::Sub, |_, this, rhs: UDim2| Ok(*this - rhs));
    }
}

impl From<&RbxUDim2> for UDim2 {
    fn from(v: &RbxUDim2) -> Self {
        UDim2 {
            x: (&v.x).into(),
            y: (&v.y).into(),
        }
    }
}

impl From<&UDim2> for RbxUDim2 {
    fn from(v: &UDim2) -> Self {
        RbxUDim2 {
            x: (&v.x).into(),
            y: (&v.y).into(),
        }
    }
}

impl FromRbxVariant for UDim2 {
    fn from_rbx_variant(variant: &RbxVariant) -> DatatypeConversionResult<Self> {
        if let RbxVariant::UDim2(u) = variant {
            Ok(u.into())
        } else {
            Err(DatatypeConversionError::FromRbxVariant {
                from: variant.variant_name(),
                to: "UDim2",
                detail: None,
            })
        }
    }
}

impl ToRbxVariant for UDim2 {
    fn to_rbx_variant(
        &self,
        desired_type: Option<RbxVariantType>,
    ) -> DatatypeConversionResult<RbxVariant> {
        if matches!(desired_type, None | Some(RbxVariantType::UDim2)) {
            Ok(RbxVariant::UDim2(self.into()))
        } else {
            Err(DatatypeConversionError::ToRbxVariant {
                to: desired_type.map(|d| d.variant_name()).unwrap_or("?"),
                from: "UDim2",
                detail: None,
            })
        }
    }
}
