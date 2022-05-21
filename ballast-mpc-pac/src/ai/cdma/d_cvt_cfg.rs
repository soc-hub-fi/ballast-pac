#[doc = "Register `D_CVT_CFG` reader"]
pub struct R(crate::R<D_CVT_CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<D_CVT_CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<D_CVT_CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<D_CVT_CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CVT_EN_A {
    #[doc = "1: `1`"]
    ENABLE = 1,
    #[doc = "0: `0`"]
    DISABLE = 0,
}
impl From<CVT_EN_A> for bool {
    #[inline(always)]
    fn from(variant: CVT_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CVT_EN` reader - "]
pub struct CVT_EN_R(crate::FieldReader<bool, CVT_EN_A>);
impl CVT_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CVT_EN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CVT_EN_A {
        match self.bits {
            true => CVT_EN_A::ENABLE,
            false => CVT_EN_A::DISABLE,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == CVT_EN_A::ENABLE
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == CVT_EN_A::DISABLE
    }
}
impl core::ops::Deref for CVT_EN_R {
    type Target = crate::FieldReader<bool, CVT_EN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CVT_TRUNCATE` reader - "]
pub struct CVT_TRUNCATE_R(crate::FieldReader<u8, u8>);
impl CVT_TRUNCATE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CVT_TRUNCATE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CVT_TRUNCATE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cvt_en(&self) -> CVT_EN_R {
        CVT_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 4:9"]
    #[inline(always)]
    pub fn cvt_truncate(&self) -> CVT_TRUNCATE_R {
        CVT_TRUNCATE_R::new(((self.bits >> 4) & 0x3f) as u8)
    }
}
#[doc = "Enable/disable input data converter in CDMA and number of bits to be truncated in the input data converter\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [d_cvt_cfg](index.html) module"]
pub struct D_CVT_CFG_SPEC;
impl crate::RegisterSpec for D_CVT_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [d_cvt_cfg::R](R) reader structure"]
impl crate::Readable for D_CVT_CFG_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets D_CVT_CFG to value 0"]
impl crate::Resettable for D_CVT_CFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
