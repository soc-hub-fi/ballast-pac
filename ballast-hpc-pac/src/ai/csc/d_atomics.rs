#[doc = "Register `D_ATOMICS` reader"]
pub struct R(crate::R<D_ATOMICS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<D_ATOMICS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<D_ATOMICS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<D_ATOMICS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `D_ATOMICS` writer"]
pub struct W(crate::W<D_ATOMICS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<D_ATOMICS_SPEC>;
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
impl From<crate::W<D_ATOMICS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<D_ATOMICS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ATOMICS` reader - "]
pub type ATOMICS_R = crate::FieldReader<u32, u32>;
#[doc = "Field `ATOMICS` writer - "]
pub type ATOMICS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, D_ATOMICS_SPEC, u32, u32, 21, O>;
impl R {
    #[doc = "Bits 0:20"]
    #[inline(always)]
    pub fn atomics(&self) -> ATOMICS_R {
        ATOMICS_R::new(self.bits & 0x001f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:20"]
    #[inline(always)]
    #[must_use]
    pub fn atomics(&mut self) -> ATOMICS_W<0> {
        ATOMICS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Equals to output_data_cube_width * output_data_cube_height - 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [d_atomics](index.html) module"]
pub struct D_ATOMICS_SPEC;
impl crate::RegisterSpec for D_ATOMICS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [d_atomics::R](R) reader structure"]
impl crate::Readable for D_ATOMICS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [d_atomics::W](W) writer structure"]
impl crate::Writable for D_ATOMICS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets D_ATOMICS to value 0x01"]
impl crate::Resettable for D_ATOMICS_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
