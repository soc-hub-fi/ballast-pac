#[doc = "Register `D_DAIN_RAM_TYPE` reader"]
pub struct R(crate::R<D_DAIN_RAM_TYPE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<D_DAIN_RAM_TYPE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<D_DAIN_RAM_TYPE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<D_DAIN_RAM_TYPE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `D_DAIN_RAM_TYPE` writer"]
pub struct W(crate::W<D_DAIN_RAM_TYPE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<D_DAIN_RAM_TYPE_SPEC>;
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
impl From<crate::W<D_DAIN_RAM_TYPE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<D_DAIN_RAM_TYPE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DATAIN_RAM_TYPE` reader - "]
pub type DATAIN_RAM_TYPE_R = crate::BitReader<DATAIN_RAM_TYPE_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DATAIN_RAM_TYPE_A {
    #[doc = "0: `0`"]
    CVIF = 0,
    #[doc = "1: `1`"]
    MCIF = 1,
}
impl From<DATAIN_RAM_TYPE_A> for bool {
    #[inline(always)]
    fn from(variant: DATAIN_RAM_TYPE_A) -> Self {
        variant as u8 != 0
    }
}
impl DATAIN_RAM_TYPE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DATAIN_RAM_TYPE_A {
        match self.bits {
            false => DATAIN_RAM_TYPE_A::CVIF,
            true => DATAIN_RAM_TYPE_A::MCIF,
        }
    }
    #[doc = "Checks if the value of the field is `CVIF`"]
    #[inline(always)]
    pub fn is_cvif(&self) -> bool {
        *self == DATAIN_RAM_TYPE_A::CVIF
    }
    #[doc = "Checks if the value of the field is `MCIF`"]
    #[inline(always)]
    pub fn is_mcif(&self) -> bool {
        *self == DATAIN_RAM_TYPE_A::MCIF
    }
}
#[doc = "Field `DATAIN_RAM_TYPE` writer - "]
pub type DATAIN_RAM_TYPE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, D_DAIN_RAM_TYPE_SPEC, DATAIN_RAM_TYPE_A, O>;
impl<'a, const O: u8> DATAIN_RAM_TYPE_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn cvif(self) -> &'a mut W {
        self.variant(DATAIN_RAM_TYPE_A::CVIF)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn mcif(self) -> &'a mut W {
        self.variant(DATAIN_RAM_TYPE_A::MCIF)
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn datain_ram_type(&self) -> DATAIN_RAM_TYPE_R {
        DATAIN_RAM_TYPE_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn datain_ram_type(&mut self) -> DATAIN_RAM_TYPE_W<0> {
        DATAIN_RAM_TYPE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Ram type of input RAM\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [d_dain_ram_type](index.html) module"]
pub struct D_DAIN_RAM_TYPE_SPEC;
impl crate::RegisterSpec for D_DAIN_RAM_TYPE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [d_dain_ram_type::R](R) reader structure"]
impl crate::Readable for D_DAIN_RAM_TYPE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [d_dain_ram_type::W](W) writer structure"]
impl crate::Writable for D_DAIN_RAM_TYPE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets D_DAIN_RAM_TYPE to value 0"]
impl crate::Resettable for D_DAIN_RAM_TYPE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
