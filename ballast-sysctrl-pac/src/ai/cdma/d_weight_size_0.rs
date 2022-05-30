#[doc = "Register `D_WEIGHT_SIZE_0` reader"]
pub struct R(crate::R<D_WEIGHT_SIZE_0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<D_WEIGHT_SIZE_0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<D_WEIGHT_SIZE_0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<D_WEIGHT_SIZE_0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `D_WEIGHT_SIZE_0` writer"]
pub struct W(crate::W<D_WEIGHT_SIZE_0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<D_WEIGHT_SIZE_0_SPEC>;
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
impl From<crate::W<D_WEIGHT_SIZE_0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<D_WEIGHT_SIZE_0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BYTE_PER_KERNEL` reader - "]
pub struct BYTE_PER_KERNEL_R(crate::FieldReader<u32>);
impl BYTE_PER_KERNEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        BYTE_PER_KERNEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BYTE_PER_KERNEL_R {
    type Target = crate::FieldReader<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BYTE_PER_KERNEL` writer - "]
pub struct BYTE_PER_KERNEL_W<'a> {
    w: &'a mut W,
}
impl<'a> BYTE_PER_KERNEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0003_ffff) | (value as u32 & 0x0003_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:17"]
    #[inline(always)]
    pub fn byte_per_kernel(&self) -> BYTE_PER_KERNEL_R {
        BYTE_PER_KERNEL_R::new((self.bits & 0x0003_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:17"]
    #[inline(always)]
    pub fn byte_per_kernel(&mut self) -> BYTE_PER_KERNEL_W {
        BYTE_PER_KERNEL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "The size of one kernel in bytes\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [d_weight_size_0](index.html) module"]
pub struct D_WEIGHT_SIZE_0_SPEC;
impl crate::RegisterSpec for D_WEIGHT_SIZE_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [d_weight_size_0::R](R) reader structure"]
impl crate::Readable for D_WEIGHT_SIZE_0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [d_weight_size_0::W](W) writer structure"]
impl crate::Writable for D_WEIGHT_SIZE_0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets D_WEIGHT_SIZE_0 to value 0"]
impl crate::Resettable for D_WEIGHT_SIZE_0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
