#[doc = "Register `D_WEIGHT_SIZE_EXT_0` reader"]
pub struct R(crate::R<D_WEIGHT_SIZE_EXT_0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<D_WEIGHT_SIZE_EXT_0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<D_WEIGHT_SIZE_EXT_0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<D_WEIGHT_SIZE_EXT_0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `D_WEIGHT_SIZE_EXT_0` writer"]
pub struct W(crate::W<D_WEIGHT_SIZE_EXT_0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<D_WEIGHT_SIZE_EXT_0_SPEC>;
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
impl From<crate::W<D_WEIGHT_SIZE_EXT_0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<D_WEIGHT_SIZE_EXT_0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WEIGHT_WIDTH_EXT` reader - "]
pub type WEIGHT_WIDTH_EXT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `WEIGHT_WIDTH_EXT` writer - "]
pub type WEIGHT_WIDTH_EXT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, D_WEIGHT_SIZE_EXT_0_SPEC, u8, u8, 5, O>;
#[doc = "Field `WEIGHT_HEIGHT_EXT` reader - "]
pub type WEIGHT_HEIGHT_EXT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `WEIGHT_HEIGHT_EXT` writer - "]
pub type WEIGHT_HEIGHT_EXT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, D_WEIGHT_SIZE_EXT_0_SPEC, u8, u8, 5, O>;
impl R {
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn weight_width_ext(&self) -> WEIGHT_WIDTH_EXT_R {
        WEIGHT_WIDTH_EXT_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 16:20"]
    #[inline(always)]
    pub fn weight_height_ext(&self) -> WEIGHT_HEIGHT_EXT_R {
        WEIGHT_HEIGHT_EXT_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4"]
    #[inline(always)]
    #[must_use]
    pub fn weight_width_ext(&mut self) -> WEIGHT_WIDTH_EXT_W<0> {
        WEIGHT_WIDTH_EXT_W::new(self)
    }
    #[doc = "Bits 16:20"]
    #[inline(always)]
    #[must_use]
    pub fn weight_height_ext(&mut self) -> WEIGHT_HEIGHT_EXT_W<16> {
        WEIGHT_HEIGHT_EXT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Weights width and height after extension\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [d_weight_size_ext_0](index.html) module"]
pub struct D_WEIGHT_SIZE_EXT_0_SPEC;
impl crate::RegisterSpec for D_WEIGHT_SIZE_EXT_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [d_weight_size_ext_0::R](R) reader structure"]
impl crate::Readable for D_WEIGHT_SIZE_EXT_0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [d_weight_size_ext_0::W](W) writer structure"]
impl crate::Writable for D_WEIGHT_SIZE_EXT_0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets D_WEIGHT_SIZE_EXT_0 to value 0"]
impl crate::Resettable for D_WEIGHT_SIZE_EXT_0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
