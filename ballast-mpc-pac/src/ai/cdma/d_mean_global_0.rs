#[doc = "Register `D_MEAN_GLOBAL_0` reader"]
pub struct R(crate::R<D_MEAN_GLOBAL_0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<D_MEAN_GLOBAL_0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<D_MEAN_GLOBAL_0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<D_MEAN_GLOBAL_0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `D_MEAN_GLOBAL_0` writer"]
pub struct W(crate::W<D_MEAN_GLOBAL_0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<D_MEAN_GLOBAL_0_SPEC>;
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
impl From<crate::W<D_MEAN_GLOBAL_0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<D_MEAN_GLOBAL_0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MEAN_RY` reader - "]
pub struct MEAN_RY_R(crate::FieldReader<u16>);
impl MEAN_RY_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        MEAN_RY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MEAN_RY_R {
    type Target = crate::FieldReader<u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MEAN_RY` writer - "]
pub struct MEAN_RY_W<'a> {
    w: &'a mut W,
}
impl<'a> MEAN_RY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
#[doc = "Field `MEAN_GU` reader - "]
pub struct MEAN_GU_R(crate::FieldReader<u16>);
impl MEAN_GU_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        MEAN_GU_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MEAN_GU_R {
    type Target = crate::FieldReader<u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MEAN_GU` writer - "]
pub struct MEAN_GU_W<'a> {
    w: &'a mut W,
}
impl<'a> MEAN_GU_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | ((value as u32 & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn mean_ry(&self) -> MEAN_RY_R {
        MEAN_RY_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn mean_gu(&self) -> MEAN_GU_R {
        MEAN_GU_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn mean_ry(&mut self) -> MEAN_RY_W {
        MEAN_RY_W { w: self }
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn mean_gu(&mut self) -> MEAN_GU_W {
        MEAN_GU_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Global mean value for red in RGB or Y in YUV Global mean value for green in RGB or U in YUV\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [d_mean_global_0](index.html) module"]
pub struct D_MEAN_GLOBAL_0_SPEC;
impl crate::RegisterSpec for D_MEAN_GLOBAL_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [d_mean_global_0::R](R) reader structure"]
impl crate::Readable for D_MEAN_GLOBAL_0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [d_mean_global_0::W](W) writer structure"]
impl crate::Writable for D_MEAN_GLOBAL_0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets D_MEAN_GLOBAL_0 to value 0"]
impl crate::Resettable for D_MEAN_GLOBAL_0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
