#[doc = "Register `D_WEIGHT_SIZE_1` reader"]
pub struct R(crate::R<D_WEIGHT_SIZE_1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<D_WEIGHT_SIZE_1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<D_WEIGHT_SIZE_1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<D_WEIGHT_SIZE_1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `D_WEIGHT_SIZE_1` writer"]
pub struct W(crate::W<D_WEIGHT_SIZE_1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<D_WEIGHT_SIZE_1_SPEC>;
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
impl From<crate::W<D_WEIGHT_SIZE_1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<D_WEIGHT_SIZE_1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WEIGHT_KERNEL` reader - "]
pub struct WEIGHT_KERNEL_R(crate::FieldReader<u16>);
impl WEIGHT_KERNEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        WEIGHT_KERNEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WEIGHT_KERNEL_R {
    type Target = crate::FieldReader<u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WEIGHT_KERNEL` writer - "]
pub struct WEIGHT_KERNEL_W<'a> {
    w: &'a mut W,
}
impl<'a> WEIGHT_KERNEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1fff) | (value as u32 & 0x1fff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:12"]
    #[inline(always)]
    pub fn weight_kernel(&self) -> WEIGHT_KERNEL_R {
        WEIGHT_KERNEL_R::new((self.bits & 0x1fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:12"]
    #[inline(always)]
    pub fn weight_kernel(&mut self) -> WEIGHT_KERNEL_W {
        WEIGHT_KERNEL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Number of kernels\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [d_weight_size_1](index.html) module"]
pub struct D_WEIGHT_SIZE_1_SPEC;
impl crate::RegisterSpec for D_WEIGHT_SIZE_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [d_weight_size_1::R](R) reader structure"]
impl crate::Readable for D_WEIGHT_SIZE_1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [d_weight_size_1::W](W) writer structure"]
impl crate::Writable for D_WEIGHT_SIZE_1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets D_WEIGHT_SIZE_1 to value 0"]
impl crate::Resettable for D_WEIGHT_SIZE_1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
