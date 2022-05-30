#[doc = "Register `D_WEIGHT_RAM_TYPE` reader"]
pub struct R(crate::R<D_WEIGHT_RAM_TYPE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<D_WEIGHT_RAM_TYPE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<D_WEIGHT_RAM_TYPE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<D_WEIGHT_RAM_TYPE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WEIGHT_RAM_TYPE_A {
    #[doc = "1: `1`"]
    MCIF = 1,
    #[doc = "0: `0`"]
    CVIF = 0,
}
impl From<WEIGHT_RAM_TYPE_A> for bool {
    #[inline(always)]
    fn from(variant: WEIGHT_RAM_TYPE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WEIGHT_RAM_TYPE` reader - "]
pub struct WEIGHT_RAM_TYPE_R(crate::FieldReader<bool>);
impl WEIGHT_RAM_TYPE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WEIGHT_RAM_TYPE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WEIGHT_RAM_TYPE_A {
        match self.bits {
            true => WEIGHT_RAM_TYPE_A::MCIF,
            false => WEIGHT_RAM_TYPE_A::CVIF,
        }
    }
    #[doc = "Checks if the value of the field is `MCIF`"]
    #[inline(always)]
    pub fn is_mcif(&self) -> bool {
        **self == WEIGHT_RAM_TYPE_A::MCIF
    }
    #[doc = "Checks if the value of the field is `CVIF`"]
    #[inline(always)]
    pub fn is_cvif(&self) -> bool {
        **self == WEIGHT_RAM_TYPE_A::CVIF
    }
}
impl core::ops::Deref for WEIGHT_RAM_TYPE_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn weight_ram_type(&self) -> WEIGHT_RAM_TYPE_R {
        WEIGHT_RAM_TYPE_R::new((self.bits & 1) != 0)
    }
}
#[doc = "Ram type of weight\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [d_weight_ram_type](index.html) module"]
pub struct D_WEIGHT_RAM_TYPE_SPEC;
impl crate::RegisterSpec for D_WEIGHT_RAM_TYPE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [d_weight_ram_type::R](R) reader structure"]
impl crate::Readable for D_WEIGHT_RAM_TYPE_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets D_WEIGHT_RAM_TYPE to value 0"]
impl crate::Resettable for D_WEIGHT_RAM_TYPE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
