#[doc = "Register `D_DAIN_RAM_TYPE` reader"]
pub struct R(crate::R<D_DAIN_RAM_TYPE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<D_DAIN_RAM_TYPE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<D_DAIN_RAM_TYPE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<D_DAIN_RAM_TYPE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DATAIN_RAM_TYPE_A {
    #[doc = "0: `0`"]
    CVIF = 0,
    #[doc = "1: `1`"]
    MCIF = 1,
}
impl From<DATAIN_RAM_TYPE_A> for bool {
    #[inline(always)]
    fn from(variant: DATAIN_RAM_TYPE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DATAIN_RAM_TYPE` reader - "]
pub struct DATAIN_RAM_TYPE_R(crate::FieldReader<bool>);
impl DATAIN_RAM_TYPE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DATAIN_RAM_TYPE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DATAIN_RAM_TYPE_A {
        match self.bits {
            false => DATAIN_RAM_TYPE_A::CVIF,
            true => DATAIN_RAM_TYPE_A::MCIF,
        }
    }
    #[doc = "Checks if the value of the field is `CVIF`"]
    #[inline(always)]
    pub fn is_cvif(&self) -> bool {
        **self == DATAIN_RAM_TYPE_A::CVIF
    }
    #[doc = "Checks if the value of the field is `MCIF`"]
    #[inline(always)]
    pub fn is_mcif(&self) -> bool {
        **self == DATAIN_RAM_TYPE_A::MCIF
    }
}
impl core::ops::Deref for DATAIN_RAM_TYPE_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn datain_ram_type(&self) -> DATAIN_RAM_TYPE_R {
        DATAIN_RAM_TYPE_R::new((self.bits & 1) != 0)
    }
}
#[doc = "Ram type of input RAM\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [d_dain_ram_type](index.html) module"]
pub struct D_DAIN_RAM_TYPE_SPEC;
impl crate::RegisterSpec for D_DAIN_RAM_TYPE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [d_dain_ram_type::R](R) reader structure"]
impl crate::Readable for D_DAIN_RAM_TYPE_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets D_DAIN_RAM_TYPE to value 0"]
impl crate::Resettable for D_DAIN_RAM_TYPE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
