#[doc = "Register `D_POOLING_PADDING_VALUE_2_CFG` reader"]
pub struct R(crate::R<D_POOLING_PADDING_VALUE_2_CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<D_POOLING_PADDING_VALUE_2_CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<D_POOLING_PADDING_VALUE_2_CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<D_POOLING_PADDING_VALUE_2_CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `D_POOLING_PADDING_VALUE_2_CFG` writer"]
pub struct W(crate::W<D_POOLING_PADDING_VALUE_2_CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<D_POOLING_PADDING_VALUE_2_CFG_SPEC>;
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
impl From<crate::W<D_POOLING_PADDING_VALUE_2_CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<D_POOLING_PADDING_VALUE_2_CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PAD_VALUE_2X` reader - "]
pub struct PAD_VALUE_2X_R(crate::FieldReader<u32, u32>);
impl PAD_VALUE_2X_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        PAD_VALUE_2X_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAD_VALUE_2X_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAD_VALUE_2X` writer - "]
pub struct PAD_VALUE_2X_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD_VALUE_2X_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0007_ffff) | (value as u32 & 0x0007_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:18"]
    #[inline(always)]
    pub fn pad_value_2x(&self) -> PAD_VALUE_2X_R {
        PAD_VALUE_2X_R::new((self.bits & 0x0007_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:18"]
    #[inline(always)]
    pub fn pad_value_2x(&mut self) -> PAD_VALUE_2X_W {
        PAD_VALUE_2X_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Padding_value*2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [d_pooling_padding_value_2_cfg](index.html) module"]
pub struct D_POOLING_PADDING_VALUE_2_CFG_SPEC;
impl crate::RegisterSpec for D_POOLING_PADDING_VALUE_2_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [d_pooling_padding_value_2_cfg::R](R) reader structure"]
impl crate::Readable for D_POOLING_PADDING_VALUE_2_CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [d_pooling_padding_value_2_cfg::W](W) writer structure"]
impl crate::Writable for D_POOLING_PADDING_VALUE_2_CFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets D_POOLING_PADDING_VALUE_2_CFG to value 0"]
impl crate::Resettable for D_POOLING_PADDING_VALUE_2_CFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
