import {ChangeDetectorRef, Component, Input, OnInit} from '@angular/core';
import {FormGroup} from "@angular/forms";

import {BaseComponent} from "@shared/bases/base.component";
import {SelectItem} from "primeng/api";
import {map, startWith, switchMap, takeUntil, tap} from "rxjs/operators";
import {WebFormService} from "@shared/services/web-form.service";
import {Observable, of} from "rxjs";
import {VncAuthMode} from '@shared/enums/web-client-auth-mode.enum';

interface FormInputVisibility {
  showUsernameInput?: boolean;
  showPasswordInput?: boolean;
}

@Component({
  selector: 'vnc-form',
  templateUrl: 'vnc-form.component.html',
  styleUrls: ['vnc-form.component.scss']
})
export class VncFormComponent extends BaseComponent implements  OnInit {

  @Input() form: FormGroup;
  @Input() inputFormData: any;

  authModeOptions: SelectItem[];

  formInputVisibility: FormInputVisibility = {
    showUsernameInput: true,
    showPasswordInput: true
  };

  constructor(private formService: WebFormService,
              private cdr: ChangeDetectorRef) {
    super();
  }

  ngOnInit(): void {
    this.addControlsToParentForm(this.inputFormData);
    this.initializeFormOptions();
  }

  private addControlsToParentForm(inputFormData?: any): void {
    if (this.form) {
      this.clearForm();

      this.formService.addControlToForm(
        this.form,
        'authMode',
        inputFormData,
        true,
        false,
        VncAuthMode.VNC_Password);

      this.subscribeToAuthModeChanges();
    }
  }

  private clearForm(): void {
    if (this.form.contains('authMode')) {
      this.form.removeControl('authMode');
    }
  }

  showUsernameInput(): boolean {
    return this.formInputVisibility.showUsernameInput;
  }

  showPasswordInput(): boolean {
    return this.formInputVisibility.showPasswordInput;
  }

  private initializeFormOptions(): void {
    this.formService.getAuthModeOptions('vnc').pipe(
      takeUntil(this.destroyed$)
    ).subscribe({
      next: (authModeOptions) => {
        this.authModeOptions = authModeOptions;
      },
      error: (error) => console.error('Error fetching dropdown options', error)
    });
  }

  private subscribeToAuthModeChanges(): void {
    this.form.get('authMode').valueChanges.pipe(
      takeUntil(this.destroyed$),
      startWith(this.form.get('authMode').value as VncAuthMode),
      switchMap((authMode) => this.getFormInputVisibility(authMode))
    ).subscribe({
      error: (error) => console.error('Error subscribing to auth mode changes', error)
    });
  }

  private getFormInputVisibility(authMode: VncAuthMode): Observable<VncAuthMode> {
    return of(this.formInputVisibility).pipe(
      tap((visibility: FormInputVisibility) => {
        const authModeAsNumber: number = +authMode;

        visibility.showUsernameInput = authModeAsNumber === VncAuthMode.Username_and_Password
        visibility.showPasswordInput = [VncAuthMode.VNC_Password, VncAuthMode.Username_and_Password].includes(authModeAsNumber);
      }),
      map(() => {
        return authMode;
      })
    );
  }
}
