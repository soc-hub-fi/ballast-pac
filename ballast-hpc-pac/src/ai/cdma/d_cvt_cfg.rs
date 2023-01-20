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
#[doc = "Register `D_CVT_CFG` writer"]
pub struct W(crate::W<D_CVT_CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<D_CVT_CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<D_CVT_CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<D_CVT_CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CVT_EN` reader - "]
pub type CVT_EN_R = crate::BitReader<CVT_EN_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl CVT_EN_R {
    #[doc = "Get enumerated values variant"]
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
        *self == CVT_EN_A::ENABLE
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CVT_EN_A::DISABLE
    }
}
#[doc = "Field `CVT_EN` writer - "]
pub type CVT_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, D_CVT_CFG_SPEC, CVT_EN_A, O>;
impl<'a, const O: u8> CVT_EN_W<'a, O> {
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CVT_EN_A::ENABLE)
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CVT_EN_A::DISABLE)
    }
}
#[doc = "Field `CVT_TRUNCATE` reader - "]
pub type CVT_TRUNCATE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CVT_TRUNCATE` writer - "]
pub type CVT_TRUNCATE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, D_CVT_CFG_SPEC, u8, u8, 6, O>;
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
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn cvt_en(&mut self) -> CVT_EN_W<0> {
        CVT_EN_W::new(self)
    }
    #[doc = "Bits 4:9"]
    #[inline(always)]
    #[must_use]
    pub fn cvt_truncate(&mut self) -> CVT_TRUNCATE_W<4> {
        CVT_TRUNCATE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Enable/disable input data converter in CDMA and number of bits to be truncated in the input data converter\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [d_cvt_cfg](index.html) module"]
pub struct D_CVT_CFG_SPEC;
impl crate::RegisterSpec for D_CVT_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [d_cvt_cfg::R](R) reader structure"]
impl crate::Readable for D_CVT_CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [d_cvt_cfg::W](W) writer structure"]
impl crate::Writable for D_CVT_CFG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets D_CVT_CFG to value 0"]
impl crate::Resettable for D_CVT_CFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
