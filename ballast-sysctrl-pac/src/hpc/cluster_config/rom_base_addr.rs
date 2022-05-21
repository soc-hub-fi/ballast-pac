#[doc = "Register `rom_base_addr` reader"]
pub struct R(crate::R<ROM_BASE_ADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ROM_BASE_ADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ROM_BASE_ADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ROM_BASE_ADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `rom_base_addr` writer"]
pub struct W(crate::W<ROM_BASE_ADDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ROM_BASE_ADDR_SPEC>;
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
impl From<crate::W<ROM_BASE_ADDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ROM_BASE_ADDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `rom_base_addr` reader - "]
pub struct ROM_BASE_ADDR_R(crate::FieldReader<u64, u64>);
impl ROM_BASE_ADDR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u64) -> Self {
        ROM_BASE_ADDR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ROM_BASE_ADDR_R {
    type Target = crate::FieldReader<u64, u64>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rom_base_addr` writer - "]
pub struct ROM_BASE_ADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> ROM_BASE_ADDR_W<'a> {
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
    pub fn rom_base_addr(&self) -> ROM_BASE_ADDR_R {
        ROM_BASE_ADDR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:63"]
    #[inline(always)]
    pub fn rom_base_addr(&mut self) -> ROM_BASE_ADDR_W {
        ROM_BASE_ADDR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u64) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Internal RAM base address\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rom_base_addr](index.html) module"]
pub struct ROM_BASE_ADDR_SPEC;
impl crate::RegisterSpec for ROM_BASE_ADDR_SPEC {
    type Ux = u64;
}
#[doc = "`read()` method returns [rom_base_addr::R](R) reader structure"]
impl crate::Readable for ROM_BASE_ADDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rom_base_addr::W](W) writer structure"]
impl crate::Writable for ROM_BASE_ADDR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets rom_base_addr to value 0"]
impl crate::Resettable for ROM_BASE_ADDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
