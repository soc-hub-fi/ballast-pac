#[doc = "Register `xoff_cpu` reader"]
pub struct R(crate::R<XOFF_CPU_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<XOFF_CPU_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<XOFF_CPU_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<XOFF_CPU_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `xoff_cpu` writer"]
pub struct W(crate::W<XOFF_CPU_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<XOFF_CPU_SPEC>;
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
impl From<crate::W<XOFF_CPU_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<XOFF_CPU_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `xoff_cpu` reader - The rising pulse of xoff_cpu signal is used to start transmit one PAUSE frame when the transmit in idle state with quanta zero, asking remote ethernet controller jump out from pause state."]
pub type XOFF_CPU_R = crate::BitReader<bool>;
#[doc = "Field `xoff_cpu` writer - The rising pulse of xoff_cpu signal is used to start transmit one PAUSE frame when the transmit in idle state with quanta zero, asking remote ethernet controller jump out from pause state."]
pub type XOFF_CPU_W<'a, const O: u8> = crate::BitWriter<'a, u32, XOFF_CPU_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - The rising pulse of xoff_cpu signal is used to start transmit one PAUSE frame when the transmit in idle state with quanta zero, asking remote ethernet controller jump out from pause state."]
    #[inline(always)]
    pub fn xoff_cpu(&self) -> XOFF_CPU_R {
        XOFF_CPU_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - The rising pulse of xoff_cpu signal is used to start transmit one PAUSE frame when the transmit in idle state with quanta zero, asking remote ethernet controller jump out from pause state."]
    #[inline(always)]
    #[must_use]
    pub fn xoff_cpu(&mut self) -> XOFF_CPU_W<0> {
        XOFF_CPU_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "The rising pulse of xoff_cpu signal is used to start transmit one PAUSE frame when the transmit in idle state with quanta zero, asking remote ethernet controller jump out from pause state.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [xoff_cpu](index.html) module"]
pub struct XOFF_CPU_SPEC;
impl crate::RegisterSpec for XOFF_CPU_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [xoff_cpu::R](R) reader structure"]
impl crate::Readable for XOFF_CPU_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [xoff_cpu::W](W) writer structure"]
impl crate::Writable for XOFF_CPU_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets xoff_cpu to value 0"]
impl crate::Resettable for XOFF_CPU_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
