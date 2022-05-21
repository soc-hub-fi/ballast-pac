#[doc = "Register `D_OP_ENABLE` reader"]
pub struct R(crate::R<D_OP_ENABLE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<D_OP_ENABLE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<D_OP_ENABLE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<D_OP_ENABLE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OP_EN_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<OP_EN_A> for bool {
    #[inline(always)]
    fn from(variant: OP_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OP_EN` reader - "]
pub struct OP_EN_R(crate::FieldReader<bool, OP_EN_A>);
impl OP_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OP_EN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OP_EN_A {
        match self.bits {
            false => OP_EN_A::DISABLE,
            true => OP_EN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == OP_EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == OP_EN_A::ENABLE
    }
}
impl core::ops::Deref for OP_EN_R {
    type Target = crate::FieldReader<bool, OP_EN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn op_en(&self) -> OP_EN_R {
        OP_EN_R::new((self.bits & 1) != 0)
    }
}
#[doc = "Set it to 1 to kick off operation for current register group\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [d_op_enable](index.html) module"]
pub struct D_OP_ENABLE_SPEC;
impl crate::RegisterSpec for D_OP_ENABLE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [d_op_enable::R](R) reader structure"]
impl crate::Readable for D_OP_ENABLE_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets D_OP_ENABLE to value 0"]
impl crate::Resettable for D_OP_ENABLE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
