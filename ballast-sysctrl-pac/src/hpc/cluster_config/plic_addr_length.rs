#[doc = "Register `plic_addr_length` reader"]
pub struct R(crate::R<PLIC_ADDR_LENGTH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PLIC_ADDR_LENGTH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PLIC_ADDR_LENGTH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PLIC_ADDR_LENGTH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `plic_addr_length` writer"]
pub struct W(crate::W<PLIC_ADDR_LENGTH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PLIC_ADDR_LENGTH_SPEC>;
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
impl From<crate::W<PLIC_ADDR_LENGTH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PLIC_ADDR_LENGTH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `plic_addr_length` reader - "]
pub struct PLIC_ADDR_LENGTH_R(crate::FieldReader<u64>);
impl PLIC_ADDR_LENGTH_R {
    #[inline(always)]
    pub(crate) fn new(bits: u64) -> Self {
        PLIC_ADDR_LENGTH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PLIC_ADDR_LENGTH_R {
    type Target = crate::FieldReader<u64>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `plic_addr_length` writer - "]
pub struct PLIC_ADDR_LENGTH_W<'a> {
    w: &'a mut W,
}
impl<'a> PLIC_ADDR_LENGTH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u64) -> &'a mut W {
        self.w.bits = value;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:63"]
    #[inline(always)]
    pub fn plic_addr_length(&self) -> PLIC_ADDR_LENGTH_R {
        PLIC_ADDR_LENGTH_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:63"]
    #[inline(always)]
    pub fn plic_addr_length(&mut self) -> PLIC_ADDR_LENGTH_W {
        PLIC_ADDR_LENGTH_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u64) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PLIC address space length\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [plic_addr_length](index.html) module"]
pub struct PLIC_ADDR_LENGTH_SPEC;
impl crate::RegisterSpec for PLIC_ADDR_LENGTH_SPEC {
    type Ux = u64;
}
#[doc = "`read()` method returns [plic_addr_length::R](R) reader structure"]
impl crate::Readable for PLIC_ADDR_LENGTH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [plic_addr_length::W](W) writer structure"]
impl crate::Writable for PLIC_ADDR_LENGTH_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets plic_addr_length to value 0"]
impl crate::Resettable for PLIC_ADDR_LENGTH_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
