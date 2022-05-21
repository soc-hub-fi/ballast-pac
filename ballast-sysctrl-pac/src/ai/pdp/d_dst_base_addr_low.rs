#[doc = "Register `D_DST_BASE_ADDR_LOW` reader"]
pub struct R(crate::R<D_DST_BASE_ADDR_LOW_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<D_DST_BASE_ADDR_LOW_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<D_DST_BASE_ADDR_LOW_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<D_DST_BASE_ADDR_LOW_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `D_DST_BASE_ADDR_LOW` writer"]
pub struct W(crate::W<D_DST_BASE_ADDR_LOW_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<D_DST_BASE_ADDR_LOW_SPEC>;
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
impl From<crate::W<D_DST_BASE_ADDR_LOW_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<D_DST_BASE_ADDR_LOW_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DST_BASE_ADDR_LOW` reader - "]
pub struct DST_BASE_ADDR_LOW_R(crate::FieldReader<u32, u32>);
impl DST_BASE_ADDR_LOW_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        DST_BASE_ADDR_LOW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DST_BASE_ADDR_LOW_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DST_BASE_ADDR_LOW` writer - "]
pub struct DST_BASE_ADDR_LOW_W<'a> {
    w: &'a mut W,
}
impl<'a> DST_BASE_ADDR_LOW_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn dst_base_addr_low(&self) -> DST_BASE_ADDR_LOW_R {
        DST_BASE_ADDR_LOW_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn dst_base_addr_low(&mut self) -> DST_BASE_ADDR_LOW_W {
        DST_BASE_ADDR_LOW_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Lower 32bits of output data address\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [d_dst_base_addr_low](index.html) module"]
pub struct D_DST_BASE_ADDR_LOW_SPEC;
impl crate::RegisterSpec for D_DST_BASE_ADDR_LOW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [d_dst_base_addr_low::R](R) reader structure"]
impl crate::Readable for D_DST_BASE_ADDR_LOW_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [d_dst_base_addr_low::W](W) writer structure"]
impl crate::Writable for D_DST_BASE_ADDR_LOW_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets D_DST_BASE_ADDR_LOW to value 0"]
impl crate::Resettable for D_DST_BASE_ADDR_LOW_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
