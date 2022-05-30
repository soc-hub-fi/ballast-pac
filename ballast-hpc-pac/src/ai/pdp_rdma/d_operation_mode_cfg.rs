#[doc = "Register `D_OPERATION_MODE_CFG` reader"]
pub struct R(crate::R<D_OPERATION_MODE_CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<D_OPERATION_MODE_CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<D_OPERATION_MODE_CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<D_OPERATION_MODE_CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `D_OPERATION_MODE_CFG` writer"]
pub struct W(crate::W<D_OPERATION_MODE_CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<D_OPERATION_MODE_CFG_SPEC>;
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
impl From<crate::W<D_OPERATION_MODE_CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<D_OPERATION_MODE_CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SPLIT_NUM` reader - "]
pub struct SPLIT_NUM_R(crate::FieldReader<u8>);
impl SPLIT_NUM_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SPLIT_NUM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPLIT_NUM_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPLIT_NUM` writer - "]
pub struct SPLIT_NUM_W<'a> {
    w: &'a mut W,
}
impl<'a> SPLIT_NUM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn split_num(&self) -> SPLIT_NUM_R {
        SPLIT_NUM_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn split_num(&mut self) -> SPLIT_NUM_W {
        SPLIT_NUM_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [d_operation_mode_cfg](index.html) module"]
pub struct D_OPERATION_MODE_CFG_SPEC;
impl crate::RegisterSpec for D_OPERATION_MODE_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [d_operation_mode_cfg::R](R) reader structure"]
impl crate::Readable for D_OPERATION_MODE_CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [d_operation_mode_cfg::W](W) writer structure"]
impl crate::Writable for D_OPERATION_MODE_CFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets D_OPERATION_MODE_CFG to value 0"]
impl crate::Resettable for D_OPERATION_MODE_CFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
