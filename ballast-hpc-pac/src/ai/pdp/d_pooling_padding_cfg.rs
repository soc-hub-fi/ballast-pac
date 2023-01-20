#[doc = "Register `D_POOLING_PADDING_CFG` reader"]
pub struct R(crate::R<D_POOLING_PADDING_CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<D_POOLING_PADDING_CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<D_POOLING_PADDING_CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<D_POOLING_PADDING_CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `D_POOLING_PADDING_CFG` writer"]
pub struct W(crate::W<D_POOLING_PADDING_CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<D_POOLING_PADDING_CFG_SPEC>;
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
impl From<crate::W<D_POOLING_PADDING_CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<D_POOLING_PADDING_CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PAD_LEFT` reader - "]
pub type PAD_LEFT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PAD_LEFT` writer - "]
pub type PAD_LEFT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, D_POOLING_PADDING_CFG_SPEC, u8, u8, 3, O>;
#[doc = "Field `PAD_TOP` reader - "]
pub type PAD_TOP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PAD_TOP` writer - "]
pub type PAD_TOP_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, D_POOLING_PADDING_CFG_SPEC, u8, u8, 3, O>;
#[doc = "Field `PAD_RIGHT` reader - "]
pub type PAD_RIGHT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PAD_RIGHT` writer - "]
pub type PAD_RIGHT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, D_POOLING_PADDING_CFG_SPEC, u8, u8, 3, O>;
#[doc = "Field `PAD_BOTTOM` reader - "]
pub type PAD_BOTTOM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PAD_BOTTOM` writer - "]
pub type PAD_BOTTOM_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, D_POOLING_PADDING_CFG_SPEC, u8, u8, 3, O>;
impl R {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn pad_left(&self) -> PAD_LEFT_R {
        PAD_LEFT_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:6"]
    #[inline(always)]
    pub fn pad_top(&self) -> PAD_TOP_R {
        PAD_TOP_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    pub fn pad_right(&self) -> PAD_RIGHT_R {
        PAD_RIGHT_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 12:14"]
    #[inline(always)]
    pub fn pad_bottom(&self) -> PAD_BOTTOM_R {
        PAD_BOTTOM_R::new(((self.bits >> 12) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    #[must_use]
    pub fn pad_left(&mut self) -> PAD_LEFT_W<0> {
        PAD_LEFT_W::new(self)
    }
    #[doc = "Bits 4:6"]
    #[inline(always)]
    #[must_use]
    pub fn pad_top(&mut self) -> PAD_TOP_W<4> {
        PAD_TOP_W::new(self)
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    #[must_use]
    pub fn pad_right(&mut self) -> PAD_RIGHT_W<8> {
        PAD_RIGHT_W::new(self)
    }
    #[doc = "Bits 12:14"]
    #[inline(always)]
    #[must_use]
    pub fn pad_bottom(&mut self) -> PAD_BOTTOM_W<12> {
        PAD_BOTTOM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Left/right/top/bottom padding size\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [d_pooling_padding_cfg](index.html) module"]
pub struct D_POOLING_PADDING_CFG_SPEC;
impl crate::RegisterSpec for D_POOLING_PADDING_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [d_pooling_padding_cfg::R](R) reader structure"]
impl crate::Readable for D_POOLING_PADDING_CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [d_pooling_padding_cfg::W](W) writer structure"]
impl crate::Writable for D_POOLING_PADDING_CFG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets D_POOLING_PADDING_CFG to value 0"]
impl crate::Resettable for D_POOLING_PADDING_CFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
